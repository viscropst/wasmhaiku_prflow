# PR Notifier for WasmHaiku
a simple demo that notify slack user when a GitHub Pull Request event was coming.

# Params on WasmHaiku
| Name  | Value |
| ----- |------ |
| Inbound Connector | Github -> Account -> Repo -> Pull Request | 
| Outbound Connector | Slack -> Account -> Channel -> send message |

# How it behaviors

## Any PR Event Coming
```
Github PR: User named <Owner of the PR> <PR Action> a pull request!
```

## Opening or Reopening Event Coming
```
Github PR: User named <Owner of the PR> <PR Action> a pull request [ as a draft ] [,
You can Merge it tell me "Github PR,Merge #<PR Number rendered>"][,
You can Stat it tell me "Github PR,Stat #<PR Number rendered>"]!
```