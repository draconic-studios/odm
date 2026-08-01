import { expect, test } from "@playwright/test";
import fs from "node:fs";
import path from "node:path";

const ROOT = path.resolve(__dirname, "..");

function htmlPages(): string[] {
  return fs
    .readdirSync(ROOT)
    .filter((f) => f.endsWith(".html"))
    .sort();
}

test("internal links resolve to existing pages or anchors", async ({ page }) => {
  const pages = htmlPages();
  const pageSet = new Set(pages);
  const broken: string[] = [];

  for (const file of pages) {
    const url = file === "index.html" ? "/" : `/${file}`;
    await page.goto(url);
    const hrefs = await page.locator("a[href]").evaluateAll((as) =>
      as.map((a) => (a as HTMLAnchorElement).getAttribute("href") || ""),
    );

    for (const href of hrefs) {
      if (!href || href.startsWith("#") || href.startsWith("mailto:")) continue;
      if (/^https?:\/\//i.test(href)) continue;
      const [rawPath, hash] = href.split("#");
      const target = rawPath === "" || rawPath === "./" ? file : path.basename(rawPath);
      if (!pageSet.has(target)) {
        broken.push(`${file} → ${href} (missing ${target})`);
        continue;
      }
      if (hash) {
        await page.goto(target === "index.html" ? "/" : `/${target}`);
        const idOk = await page.locator(`[id="${CSS.escape(hash)}"]`).count();
        if (idOk === 0) broken.push(`${file} → ${href} (missing #${hash})`);
      }
    }
  }

  expect(broken, broken.join("\n")).toEqual([]);
});
