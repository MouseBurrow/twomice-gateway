# TwoMice

Anonymous Imageboard-Style Social Media Platform
A web-based social media that combines the anonymity and topic focused discussion of image boards with friend
connectivity. Users remain anonymous to each other until they mutually befriend each other.

## Features

- ### User System
    - Name/Password matched accounts
    - Profile customization
    - Friendship system (mutual reveal of profile details)
    - Direct messaging between friends
- ### Content System
    - Create posts with images and tags
    - Scroll through public posts or search via tag
    - Comment on posts or reply comments
- ### Anonymity Layer
    - Usernames, friend lists, and post histories hidden by default
    - Profile details revealed only after mutual friendship
- ### Moderation
    - Report posts or users linked to certain posts/comments
    - Moderators can check reports and do the necessary actions such as mute, ban, or kick users
    - Moderators only see profile details when handling active reports

## Project Goals

- Create an anonym platform that focuses on topics and conversations without the pressure of popularity or engagement.
- Being able to have meaningful and helpful interactions while preserving user safety and anonymity.

## Technology Stack

- ### Frontend
    - Website user interface
    - Responsive design for desktop and mobile
- ### Backend
    - RESTful
    - Account management, session handling, anonymity management, moderation logic
- ### Database
    - PostgreSQL
