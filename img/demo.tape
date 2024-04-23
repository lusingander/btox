Set Shell zsh

Output ./img/demo.gif

Set FontSize 15
Set Width 1000
Set Height 500
Set Padding 15
Set WindowBar Colorful
Set BorderRadius 20

Set Theme "iceberg-dark"

Hide

Type "cargo run"
Enter
Sleep 2s

Show

Sleep 500ms
Tab

# uuid
Sleep 500ms
Ctrl+N
Sleep 500ms
Type "l"
Sleep 500ms
Ctrl+N
Sleep 500ms
Ctrl+N
Sleep 500ms
Type@200ms "llll"
Sleep 500ms
Enter
Sleep 1s
Screenshot ./img/uuid.png

Tab
Sleep 500ms
Ctrl+N
Sleep 500ms
Tab

# hash
Sleep 500ms
Ctrl+N
Sleep 500ms
Ctrl+N
Sleep 500ms
Copy "btox"
Type "p"
Sleep 500ms
Ctrl+P
Sleep 500ms
Ctrl+P
Sleep 500ms
Type@500ms "lll"
Sleep 1s
Screenshot ./img/hash.png

Tab
Sleep 500ms
Ctrl+N
Sleep 500ms
Tab

# unix time
Sleep 500ms
Type "e"
Sleep 500ms
Type@100ms "1711976400"
Escape
Sleep 500ms
Ctrl+N
Sleep 500ms
Ctrl+N
Sleep 500ms
Type "l"
Ctrl+P
Sleep 500ms
Ctrl+P
Sleep 500ms
Type "e"
Sleep 500ms
Type@100ms "123"
Sleep 500ms
Type@100ms "456"
Escape
Sleep 1s
Screenshot ./img/unixtime.png

Tab
Sleep 500ms
Ctrl+N
Sleep 500ms
Tab

# number base
Sleep 500ms
Ctrl+N
Sleep 500ms
Ctrl+N
Sleep 500ms
Type "e"
Sleep 500ms
Type@100ms "1234"
Sleep 500ms
Escape
Sleep 1s
Ctrl+P
Sleep 500ms
Type "e"
Sleep 500ms
Type@100ms "567"
Escape
Sleep 1s
Screenshot ./img/numberbase.png