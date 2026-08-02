import { expect, type Page, test } from "@playwright/test";

/** Every public HTML page under website/ (basename without .html). */
const PAGES = [
  "index",
  "install",
  "quickstart",
  "concepts",
  "features",
  "guides",
  "guide-workspace",
  "guide-projects",
  "guide-progen",
  "guide-worktrees",
  "guide-actions",
  "guide-agents",
  "cli",
  "config",
] as const;

type PageSlug = (typeof PAGES)[number];

function pathFor(slug: PageSlug): string {
  return slug === "index" ? "/" : `/${slug}.html`;
}

/** `serve` may rewrite `foo.html` → `/foo`; accept either. */
function urlMatchesSlug(slug: PageSlug): RegExp {
  if (slug === "index") return /\/(?:index\.html)?$/;
  return new RegExp(`/${slug}(?:\\.html)?$`);
}

function urlMatchesHref(href: string): RegExp {
  const base = href.replace(/\.html$/, "");
  return new RegExp(`/${base}(?:\\.html)?$`);
}

function siteNav(page: Page) {
  return page.getByRole("navigation", { name: "Site" });
}

async function expectNoPageErrors(page: Page, run: () => Promise<void>) {
  const errors: string[] = [];
  const onError = (err: Error) => errors.push(err.message);
  page.on("pageerror", onError);
  try {
    await run();
  } finally {
    page.off("pageerror", onError);
  }
  expect(errors, `page errors: ${errors.join("; ")}`).toEqual([]);
}

async function expectCssOk(page: Page) {
  const res = await page.request.get("/assets/style.css");
  expect(res.ok(), `style.css status ${res.status()}`).toBeTruthy();
}

test.describe("all pages load", () => {
  for (const slug of PAGES) {
    test(`${slug} returns content with h1`, async ({ page }) => {
      const res = await page.goto(pathFor(slug));
      expect(res?.ok(), `${slug} status`).toBeTruthy();
      await expect(page.locator("main h1").first()).toBeVisible();
      await expect(page.locator('link[rel="stylesheet"][href="assets/style.css"]')).toHaveCount(1);
    });
  }
});

test.describe("home", () => {
  test("title, lead, CTAs", async ({ page }) => {
    await expectNoPageErrors(page, async () => {
      await page.goto("/");
      await expect(page).toHaveTitle(/ODM/);
      await expect(page.getByRole("heading", { level: 1 })).toContainText(/desk|repos|agents/i);
      await expect(page.locator("p.lead")).toContainText(/ODM|Workspace|Project|Progen/i);
      await expect(page.getByRole("link", { name: "Install ODM" })).toHaveAttribute(
        "href",
        "install.html",
      );
      await expect(page.getByRole("link", { name: "Quickstart" }).first()).toHaveAttribute(
        "href",
        "quickstart.html",
      );
      await expect(
        page.getByRole("link", { name: /View on GitHub|GitHub/ }).first(),
      ).toBeVisible();
    });
    await expectCssOk(page);
  });
});

test.describe("nav", () => {
  const primary: { label: string | RegExp; slug: PageSlug }[] = [
    { label: "Home", slug: "index" },
    { label: "Install", slug: "install" },
    { label: "Quickstart", slug: "quickstart" },
    { label: "Concepts", slug: "concepts" },
    { label: "Features", slug: "features" },
    { label: "Guides overview", slug: "guides" },
    { label: "CLI", slug: "cli" },
    { label: "Config shape", slug: "config" },
  ];

  test("primary nav targets from home + aria-current", async ({ page }) => {
    await page.goto("/");
    for (const { label, slug } of primary) {
      await siteNav(page).getByRole("link", { name: label, exact: true }).click();
      await expect(page.locator("main h1").first()).toBeVisible();
      await expect(page).toHaveURL(urlMatchesSlug(slug));
      await expect(
        siteNav(page).locator('a[aria-current="page"]'),
      ).toHaveCount(1);
      const current = siteNav(page).locator('a[aria-current="page"]');
      await expect(current).toHaveAttribute(
        "href",
        slug === "index" ? "index.html" : `${slug}.html`,
      );
    }
  });
});

test.describe("install", () => {
  test("curl primary, releases download, source secondary", async ({ page }) => {
    await expectNoPageErrors(page, async () => {
      await page.goto("/install.html");
      await expect(page.getByRole("heading", { name: "Install", exact: true })).toBeVisible();
      await expect(page.getByRole("heading", { name: /Quick install/i })).toBeVisible();
      await expect(page.locator("pre code").filter({ hasText: "install.sh" }).first()).toBeVisible();
      await expect(page.getByRole("heading", { name: /GitHub Releases/i })).toBeVisible();
      await expect(page.getByRole("heading", { name: /Build from source/i })).toBeVisible();
      await expect(page.getByText(/~\/\.local\/bin/i).first()).toBeVisible();
      await expect(page.getByText(/SHA256/i).first()).toBeVisible();
      await expect(
        page.getByRole("link", { name: /github\.com\/hembrow-innovations\/odm\/releases/i }),
      ).toHaveAttribute("href", "https://github.com/hembrow-innovations/odm/releases");
    });
  });
});

test.describe("quickstart", () => {
  test("odm init in a code block", async ({ page }) => {
    await page.goto("/quickstart.html");
    await expect(page.getByRole("heading", { name: "Quickstart", exact: true })).toBeVisible();
    await expect(page.locator("pre code").filter({ hasText: "odm init" }).first()).toBeVisible();
  });
});

test.describe("concepts", () => {
  test("Workspace, Project, Progen terms", async ({ page }) => {
    await page.goto("/concepts.html");
    const main = page.locator("main");
    await expect(main.getByText("Workspace", { exact: true }).first()).toBeVisible();
    await expect(main.getByText("Project", { exact: true }).first()).toBeVisible();
    await expect(main.getByText("Progen", { exact: true }).first()).toBeVisible();
  });
});

test.describe("features", () => {
  test("shipped vs sketch; agent start shipped", async ({ page }) => {
    await page.goto("/features.html");
    await expect(page.getByRole("heading", { name: /^Shipped/ })).toBeVisible();
    await expect(page.getByRole("heading", { name: /Sketch|deferred/i })).toBeVisible();
    await expect(page.getByText("not shipped").first()).toBeVisible();
    await expect(page.locator("main")).toContainText("Agent start (one-shot)");
    await expect(page.locator("main")).toContainText("runtime brand matrix");
  });
});

test.describe("guides", () => {
  const guideLinks: { href: string; h1: RegExp }[] = [
    { href: "guide-workspace.html", h1: /Workspace/i },
    { href: "guide-projects.html", h1: /Projects/i },
    { href: "guide-progen.html", h1: /Progen/i },
    { href: "guide-worktrees.html", h1: /Worktree/i },
    { href: "guide-actions.html", h1: /Actions/i },
    { href: "guide-agents.html", h1: /Agent packs/i },
  ];

  test("hub loads and guide links resolve", async ({ page }) => {
    await page.goto("/guides.html");
    await expect(page.getByRole("heading", { level: 1 })).toContainText(/guide/i);
    for (const { href, h1 } of guideLinks) {
      await page.goto("/guides.html");
      await page.locator(`main a[href="${href}"]`).first().click();
      await expect(page).toHaveURL(urlMatchesHref(href));
      await expect(page.locator("main h1")).toContainText(h1);
    }
  });
});

test.describe("cli and config", () => {
  test("cli surface loads", async ({ page }) => {
    await page.goto("/cli.html");
    await expect(page.getByRole("heading", { level: 1 })).toContainText(/CLI/i);
    await expect(page.locator("main")).toContainText("odm init");
  });

  test("config example / keys visible", async ({ page }) => {
    await page.goto("/config.html");
    await expect(page.getByRole("heading", { name: /Config/i })).toBeVisible();
    await expect(page.locator("main")).toContainText("odm.config.yaml");
    await expect(page.locator("main")).toContainText("projects");
    await expect(page.locator("pre code").first()).toBeVisible();
  });

  test("css ok on deep page", async ({ page }) => {
    await page.goto("/guide-agents.html");
    await expectCssOk(page);
  });
});

test.describe("mobile smoke", () => {
  test.use({ viewport: { width: 390, height: 844 } });

  test("home readable at mobile width", async ({ page }) => {
    await page.goto("/");
    const h1 = page.getByRole("heading", { level: 1 });
    await expect(h1).toBeVisible();
    const box = await h1.boundingBox();
    expect(box).toBeTruthy();
    expect(box!.width).toBeGreaterThan(100);
    await expect(page.locator("main")).toBeVisible();
  });
});
