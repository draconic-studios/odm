import { expect, test } from "@playwright/test";

/** Minimal harness check from setup (116); full matrix in smoke.spec.ts. */
test("home title contains ODM", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle(/ODM/);
});
