# From Yew

This is the
[server](https://github.com/yewstack/yew/tree/master/examples/server)
from the yew examples directory, with one modification.  When a
message comes in, it's echoed back twice, rather than once.  The
second message is what triggers the race condition.