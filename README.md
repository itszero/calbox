Calbox
=======

A tiny calendar event server.

## Why?

I have a couple side-projects that needs access to calendar events (such as [HomePad](https://github.com/itszero/HomePad)). However, it is time-consuming and painful to implement API connection to many calendar sources. Not to mention if you're on a corp account, they may not allow 3rd-party apps anyway.

Instead, I used [a iCloud shortcut](https://www.icloud.com/shortcuts/6e35ad20be21433a8784e2fa20d9e4fc) to upload calendar events from my phone to this calendar server and pull it within my home network from my side projects.

## Features

- Events are stored in memory
- Events are cleaned up as it expires
