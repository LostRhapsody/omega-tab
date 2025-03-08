# Jira Integration

BetterNewTab's Jira integration enables you to search and access your Jira issues directly from your new tab, making your workflow more efficient.

::: warning
This feature is not yet available and these steps are not finalized.

Please contact support if you have further questions.
:::

## Setting Up the Jira Integration

### Prerequisites
- You need to have a Jira account with API access permissions
- Ability to create an API token for your Jira account

### Configuration Steps

1. In BetterNewTab, open the **User Settings** panel by clicking on the gear icon in the top-right corner
2. Navigate to the **Integrations** tab
3. Find the Jira section and click **Configure**
4. Enter the following information:
   - **Jira Domain**: Your Jira instance URL (e.g., `https://yourcompany.atlassian.net`)
   - **API Token**: Your Jira API token
   - **Email**: The email associated with your Jira account

## Using the Jira Integration

Once configured, you'll be able to:

- Search for Jira issues directly from BetterNewTab's search bar by using the prefix `j:` or `jira:`
- View matching Jira issues as suggestions when typing a search query
- Click on any search result to open the corresponding Jira issue
- See key information about issues including status, assignee, and priority

### Example Searches

- `j:BNT-123` - Search for a specific issue by key
- `jira:login bug` - Search for issues containing "login bug"
- `j:assigned to me` - Find issues assigned to you

## Managing Your Integration

You can update or remove your Jira integration at any time:

1. Go to **User Settings** > **Integrations**
2. Find the Jira section
3. Click **Edit** to update your credentials or **Remove** to disable the integration

## Troubleshooting

If you're experiencing issues with the Jira integration:

- Ensure your API token is valid and hasn't expired
- Check that you have the correct permissions in Jira
- Verify that your Jira domain URL is correct
- Make sure you've entered the email address associated with your Jira account