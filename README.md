# gmock-sed

Simple CLI tool for updating gMock macros. (MOCK\_METHODn -> MOCK\_METHOD)

*See:* [gMock Cookbook - Old-Style MOCK\_METHODn Macros](https://github.com/google/googletest/blob/master/googlemock/docs/cook_book.md#old-style-mock_methodn-macros)


## Subcommands

### Search

Locate files that contain at least 1 old-style MOCK\_METHDODn macro.

```
gmock-sed search mocks/
```

### Replace

Substitute old-style macros with equivalent new-style macros.

```
gmock-sed replace MockFoo.h
```

**Warning**: `gmock-sed replace` is destructive. Use version control or risk data loss.


### Search and Replace

If you're feeling brave...

```
gmock-sed replace $(gmock-sed search mocks/)
```

**Warning**: `gmock-sed replace` is destructive. Use version control or risk data loss.

