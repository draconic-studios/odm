import { expect, test } from "@playwright/test";

test("home title contains ODM", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle(/ODM/);
});
