import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

const SCAN = ["/", "/install.html", "/guide-workspace.html"] as const;

test.describe("a11y axe", () => {
  for (const path of SCAN) {
    test(`${path} has no serious/critical violations`, async ({ page }) => {
      await page.goto(path);
      const results = await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
        .analyze();
      const bad = results.violations.filter(
        (v) => v.impact === "serious" || v.impact === "critical",
      );
      const summary = bad
        .map(
          (v) =>
            `${v.id} (${v.impact}): ${v.nodes.length} node(s) — ${v.help}\n` +
            v.nodes
              .slice(0, 5)
              .map((n) => `  - ${n.target.join(" ")}`)
              .join("\n"),
        )
        .join("\n");
      expect(bad, summary || "ok").toEqual([]);
    });
  }
});

test("skip link targets main", async ({ page }) => {
  await page.goto("/");
  const skip = page.locator("a.skip");
  await expect(skip).toHaveAttribute("href", "#main");
  await expect(page.locator("main#main")).toHaveCount(1);
});

test("home start-here path is present", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "What is ODM?" })).toBeVisible();
  await expect(page.getByRole("heading", { name: "Start here" })).toBeVisible();
  const steps = page.locator("ol.path-steps li");
  await expect(steps).toHaveCount(3);
  await expect(steps.nth(0).getByRole("link", { name: "Install" })).toHaveAttribute(
    "href",
    "install.html",
  );
  await expect(steps.nth(1).getByRole("link", { name: "Quickstart" })).toHaveAttribute(
    "href",
    "quickstart.html",
  );
});

test("install verify CTA points to quickstart", async ({ page }) => {
  await page.goto("/install.html");
  await expect(page.getByRole("heading", { name: "Verify" })).toBeVisible();
  await expect(page.getByRole("link", { name: /Next: Quickstart/i })).toHaveAttribute(
    "href",
    "quickstart.html",
  );
});
