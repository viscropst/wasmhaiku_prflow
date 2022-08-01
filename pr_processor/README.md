# PR Processor for WasmHaiku
a simple demo that process message from slack user related to GitHub Pull Request operations.

# Params on WasmHaiku
| Name  | Value |
| ----- |------ |
| Inbound Connector | Slack -> Account -> Channel -> get message | 
| Outbound Connector | Slack -> Account -> Channel -> send message |

# How it behaviors

## Stat a Pull Request number 3 (TODO)
Send> `Github PR,Stat #3`

## Merge a Pull Request number 3 (TODO)
Send> `Github PR,Merge #3`