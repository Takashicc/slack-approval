# slack-approval

> [!WARN]
> UNDER CONSTRUCTION!
> Please reference the [older version README](https://github.com/Takashicc/slack-approval/blob/298fa3048bf704e769b8195396433c094b5d9668/README.md).
> Also, Use the `Takashicc/slack-approval@v1.1.0`. NOT `main`.

Custom action to send approval request to Slack

![](/img/approval.png)

- Post a message in Slack with a "Approve" and "Reject" buttons.
- Clicking on "Approve" will execute next steps.
- Clicking on "Reject" will cause workflow to fail.

## How To Use

- First, create a Slack App and install in your workspace.
- Second, add `chat:write` and `im:write` to OAuth Scope on OAuth & Permissions page.
    > [!NOTE]
    > When you want to use `authorized-groups`, you must add `usergroups:read` too.
- Finally, **Enable Socket Mode**.

```yml
jobs:
  approval:
    runs-on: ubuntu-latest
    steps:
      - name: send approval
        uses: Takashicc/slack-approval@main
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

- Set args
  - `bot-token`
    - Bot-level tokens on `OAuth & Permissions page`. (starting with `xoxb-` )
  - `app-token`
    - App-level tokens on `Basic Information page`. (starting with `xapp-` )
  - `channel-id`
    - Channel ID for which you want to send approval.
  - `mention-to-users`
    - Optional. Slack user IDs to mention. Comma separated.
  - `mention-to-groups`
    - Optional. Slack group IDs to mention. Comma separated.
  - `authorized-users`
    - Optional. Slack user IDs who are authorized to approve or reject. Comma separated.
  - `authorized-groups`
    - Optional. Slack group IDs who are authorized to approve or reject. Comma separated.

- Set `timeout-minutes`
  - Set the time to wait for approval.
