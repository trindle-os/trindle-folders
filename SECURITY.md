# Security policy

Trindle Folders is a small project kept up in the time available, not a team with a dedicated security function. This page sets realistic expectations.

## Supported versions

Only the latest release is supported. There is no long-term-support branch and no backporting. A fix lands in the next release and you are expected to update.

| Version | Supported |
|---|---|
| 0.1.x | yes |

## Reporting a vulnerability

**Please do not put the details of a security problem in a public issue.** A public report tells everyone, including people running an unpatched copy, before a fix exists.

Use GitHub's private vulnerability reporting on this repository, under **Security**, then **Report a vulnerability**. Only the maintainers can see it.

If you would rather not use GitHub, write to <folders@trindle.org> with "Trindle Folders security" in the subject line and say only that you have a security issue. Leave the details out of that first message and a private channel will be arranged.

## What to expect

Replies and fixes are best effort, not guaranteed against any service level. You will be told when the report is read, and when a fix is released.

## What is in scope

Folders handles files that strangers made. A fault in how it reads an archive, a picture, an SVG, a font, a film or a file name, follows a link it should not, writes outside the place it was told to, deletes without the proof it asks for, or mishandles a login for a server, share or drive is in scope. Archives, SVGs, PDFs and films are read in a box (bubblewrap) that has no network and sees none of the person's files; a way out of that box is the most serious kind of report. Pictures and fonts are read by Folders itself, not in the box; a font is size-limited and read by a parser that forbids unsafe code, and a fault in either is in scope all the same.
