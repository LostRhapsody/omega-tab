# Plan to create a single executable for distributing the app

## Step 1: DB

Right now we're using POSTGRES with a docker. We need to switch to sqlite so it can be easily embedded, with no DB server required.

Sqlx should have no problem switching, so it's more of a matter of set up.

## Step 2: server

The server needs to be able to serve the frontend assets, right now it only serves the API.

## Step 3: Installer

We need to host a simple installer that just downloads the executable.

## Step 4: Branding

Just need to make sure the executable has a proper name and icon when distributed.

## Step 5: System Tray

When possible, make the app run in the background in the system tray. We'll just need it to have a exit option when right clicked. The icon should also be our logo.