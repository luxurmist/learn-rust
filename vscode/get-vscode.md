<!--
	file: vscode.md
	author: luxurmist
	date: 2026-04-01
-->


# VSCode使用

## 1. 设置 setting
### 窗口和编辑器打开后的还原方式
```json
"window.restoreWindows": "none"
```

## 3. 快捷键

### 补全切换

```json
[
    {
        "key": "ctrl+oem_1",
        "command": "selectNextSuggestion",
        "when": "suggestWidgetMultipleSuggestions && suggestWidgetVisible && textInputFocus || suggestWidgetVisible && textInputFocus && !suggestWidgetHasFocusedSuggestion"
    },
    {
        "key": "ctrl+oem_7",
        "command": "selectPrevSuggestion",
        "when": "suggestWidgetMultipleSuggestions && suggestWidgetVisible && textInputFocus || suggestWidgetVisible && textInputFocus && !suggestWidgetHasFocusedSuggestion"
    }
]
```

## 2. 代码片段 snippets

```json
	"Test snippet": {
		"scope": "javascript,typescript",
		"prefix": "test",
		"body": "test('$1', () => {\n\t$0\n});",
		"include": ["**/*.test.ts", "*.spec.ts"],
		"exclude": ["**/temp/*.ts"],
		"description": "Insert test block"
	}

```

