# Confluence Integration

BetterNewTab's Confluence integration allows you to quickly search and access your Confluence Account's spaces and pages directly from BetterNewTab.

::: warning
This feature is not yet available and these steps are not finalized.

Please contact support if you have further questions.
:::

## Setting Up the Confluence Integration

### Prerequisites
- You need to have a Confluence account with API access permissions
- Access to generate an API token for your Confluence account

### Configuration Steps

1. In BetterNewTab, open the **User Settings** panel by clicking on the gear icon in the top-right corner
2. Navigate to the **Integrations** tab
3. Find the Confluence section and click **Configure**
4. Enter the following information:
   - **Confluence Domain**: Your Confluence instance URL (e.g., `https://yourcompany.atlassian.net/wiki`)
   - **API Token**: Your Confluence API token
   - **Email**: The email associated with your Confluence account

## Using the Confluence Integration

Once configured, you'll be able to:

- Search for Confluence pages directly from BetterNewTab's search bar by using the prefix `c:` or `confluence:`
- See matching Confluence pages as suggestions when typing a search query
- Click on any search result to open the corresponding Confluence page

### Example Searches

- `c:marketing plan` - Search for "marketing plan" in your Confluence spaces
- `confluence:project roadmap` - Search for "project roadmap" in your Confluence spaces

## Managing Your Integration

You can update or remove your Confluence integration at any time:

1. Go to **User Settings** > **Integrations**
2. Find the Confluence section
3. Click **Edit** to update your credentials or **Remove** to disable the integration

## Troubleshooting

If you're experiencing issues with the Confluence integration:

- Ensure your API token is valid and hasn't expired
- Check that you have the correct permissions in Confluence
- Verify that your Confluence domain URL is correct