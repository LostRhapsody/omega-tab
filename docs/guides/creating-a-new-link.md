---
title: Creating a New Link
description: Learn how to add new links to your OmegaTab dashboard
---

# Creating a New Link

Adding your favorite websites to OmegaTab is quick and simple. This guide will walk you through the process of creating a new link card.

## Ways to Add a New Link

There are three ways to add a new link to your dashboard:

1. Click on the **Add new link** card in your dashboard
2. Use the keyboard shortcut `Alt+N`
3. Open the Command Palette with `Ctrl+K` and type or select "Add New Link"

## Adding a Link Step-by-Step

### 1. Open the Add New Link Modal

Use one of the above methods to open a modal dialog where you can enter your link details.

![Add Link Card](/copy/add_link.png)

### 2. Enter Link Details

The Add New Link form contains the following fields:

- **URL** (required): The complete web address of the site you want to add, you don't need to enter the protocol (`https://`).
    - Example: `omega-tab.evanrobertson.dev`
- **Title** (optional): A name for your link
- **Description** (optional): Additional information about the link
- **Column Label** (required): The category where this link should appear

::: tip Metadata fetching
If you leave the optional fields blank, OmegaTab will attempt to fetch them automatically from the link's website, along with an icon.

If you don't like the metadata that was fetched (some websites prioritize [SEO](https://www.wix.com/blog//what-is-seo?experiment_id=%5E%5E788f2489-56c0-4af8-b1b8-a1650a43b85d%5E) and their descriptions or titles aren't the prettiest), you can also edit it later.
:::

### 3. Column Organization

The "Column Label" field helps organize your links into groups. You can:

- Select an existing column from the dropdown
- Create a new column by selecting the dropdown and entering a new column name at the bottom


::: tip Column Management
You can move links from one column to other by editing them and changing their column label in the dropdown. After no more links are saved under a column label, it will be deleted. Learn more in [the edit guide](/guides/editing-a-link#column-management).
:::

### 4. Save Your Link

Click the "Add Link" button to save your new link to the dashboard. Typing `Shift+Enter` when typing in any of the form's input fields (Url, Title, Description) will also save the link, saving you some time.

## Keyboard Shortcuts

While adding a link, you can use these keyboard shortcuts:

- `Shift+Enter`: Submit the form 

## Next Steps

After adding links, you might want to learn how to:
- [Edit your existing links](/guides/editing-a-link)
- [Understand how metadata fetching works](/guides/metadata-fetching)