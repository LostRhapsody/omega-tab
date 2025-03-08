---
title: Metadata Fetching
description: Learn how BetterNewTab automatically retrieves website information
---

# Metadata Fetching

BetterNewTab includes a smart metadata fetching feature that automatically retrieves website information when you add a new link. This guide explains how it works and what to expect.

## What Is Metadata Fetching?

When you add a new link to your dashboard without specifying a title or description, BetterNewTab can automatically retrieve:

- Website title
- Description from meta tags
- Favicon or site icon

This saves you time and ensures your link cards contain relevant information.

## How It Works

The metadata fetching process works as follows:

1. You enter a URL in the "Add New Link" form
2. You leave the title and/or description fields empty
3. When you save the link, BetterNewTab connects to the website
4. Our backend server parses the website's HTML content
5. The server extracts meta tags, title tags, and icon information
6. This information is used to populate your link card

## Availability

Metadata fetching is a **Plus Feature** and is available only to Plus or Pro subscribers. For free users, any fields left blank will remain blank, and you'll need to manually enter all details.

## What Gets Extracted

The system attempts to extract:

1. **Title**: Usually from the `<title>` tag or OpenGraph `og:title` meta tag
2. **Description**: From the meta description tag or OpenGraph `og:description` tag
3. **Icon**: From favicon links, Apple touch icons, or website root favicon.ico

## Limitations

While metadata fetching is convenient, there are some limitations to be aware of:

- Some websites block automated requests, which may prevent metadata retrieval
- The quality of fetched data depends on how well the website has implemented meta tags
- Dynamic sites that load content via JavaScript may not have all metadata available
- Very large pages may timeout during the fetching process

If the automatic fetching doesn't produce the results you want, you can always manually edit the link afterward.

## Editing After Fetching

After metadata has been fetched:

1. Use the [edit link](/guides/editing-a-link) feature to modify any auto-fetched data
2. Note that editing a link will not trigger metadata fetching again
3. Changes you make will override any previously fetched data

## Next Steps

After learning about metadata fetching, you might want to:
- [Create your first link](/guides/creating-a-new-link) to see it in action
- [Explore user settings](/guides/user-settings) to configure metadata fetching