import { expect, test } from "@playwright/test";

test("homepage renders base UI", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "service.auth web" })).toBeVisible();
  await expect(page.getByText("Environment:")).toBeVisible();
});

test("health check shows ok", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("API health:")).toBeVisible();
  await expect(page.getByText("ok")).toBeVisible({ timeout: 10_000 });
});

test("crud happy path create and read", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Create + Read account" }).click();
  await expect(page.getByText("Created account uid:")).toBeVisible({ timeout: 10_000 });
  await expect(page.getByText("Fetched account")).toBeVisible({ timeout: 10_000 });
});
