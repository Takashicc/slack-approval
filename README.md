# slack-approval

Custom action to send approval request to Slack.

![](/img/approval.png)

- Post a message in Slack with a "Approve" and "Reject" buttons.
- Clicking on "Approve" will execute next steps.
- Clicking on "Reject" will cause workflow to fail.

## How To Use

1. Create a Slack App and install in your workspace.
2. Add `chat:write` and `im:write` to OAuth Scope on OAuth & Permissions page.
   1. (Optional) When you want to use `authorized-groups`, you must add `usergroups:read` too.
3. Finally, **Enable Socket Mode**.

```yml
jobs:
  approval:
    runs-on: ubuntu-latest
    steps:
      - uses: Takashicc/slack-approval@v2.0.2
        with:
          bot-token: ${{ secrets.SLACK_BOT_TOKEN }}
          app-token: ${{ secrets.SLACK_APP_TOKEN }}
          channel-id: ${{ secrets.SLACK_CHANNEL_ID }}
          mention-to-users: ${{ secrets.SLACK_MENTION_TO_USERS }}
          mention-to-groups: ${{ secrets.SLACK_MENTION_TO_GROUPS }}
          authorized-users: ${{ secrets.SLACK_AUTHORIZED_USERS }}
          authorized-groups: ${{ secrets.SLACK_AUTHORIZED_GROUPS }}
        timeout-minutes: 10
```

- About parameters
  - Required
    - `bot-token`
      - Bot-level tokens on `OAuth & Permissions page`. (starting with `xoxb-` )
    - `app-token`
      - App-level tokens on `Basic Information page`. (starting with `xapp-` )
    - `channel-id`
      - Channel ID for which you want to send approval.
  - Optional
    - `mention-to-users`
      - Slack user IDs to mention. Comma separated.
    - `mention-to-groups`
      - Slack group IDs to mention. Comma separated.
    - `authorized-users`
      - Slack user IDs who are authorized to approve or reject. Comma separated.
    - `authorized-groups`
      - Slack group IDs who are authorized to approve or reject. Comma separated.

- `timeout-minutes`
  - Set the time to wait for approval.
