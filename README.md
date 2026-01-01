# Box System
A box system for the the tons of boxes my family has.

## Roadmap
- [ ] Backend
    - [ ] System to create QR codes
    - [ ] JSON storage for uuids and box information
    - [ ] Ability to get all box information
    - [ ] Ability to look up box information by uuid
- [ ] WebUI
    - [ ] It exists!
    - [ ] Ability to see existing boxes (list view)
    - [ ] Ability to create a new box and qr code
        - [ ] QR codes can be easily printed
    - [ ] Ability to edit existing box information
    - [ ] Ability scan a QR code to see a specifc box
- [ ] Docker
    - [ ] Build docker container
    - [ ] Upload it to forgejo packages

## How will it work?
Every QR code will contain a uuid which corresponds to what's in the box in the backend, if you change the information in the backend, the QR code will now point to the new data, removing the need for relabeling.
