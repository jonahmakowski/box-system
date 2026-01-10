# Box System
A box system for the the tons of boxes my family has.

## Roadmap
- [ ] Backend
    - [x] System to create QR codes
    - [x] JSON storage for uuids and box information
    - [x] Ability to get all box information
    - [x] Ability to look up box information by uuid
- [ ] WebUI
    - [x] It exists!
    - [x] Ability to see existing boxes (list view)
    - [ ] Ability to create a new box and qr code
    - [x] QR codes can be easily printed
        - They're accessible on the webui as pdfs
    - [ ] Ability to edit existing box information
    - [ ] Ability scan a QR code to see a specifc box
        - [x] Can take photos
        - [x] Photos can go to the backend
- [ ] Docker
    - [ ] Build docker container
    - [ ] Upload it to forgejo packages
    - [ ] Upload it to docker hub

## How will it work?
Every QR code will contain a uuid which corresponds to what's in the box in the backend, if you change the information in the backend, the QR code will now point to the new data, removing the need for relabeling.
