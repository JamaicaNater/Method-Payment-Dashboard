# Method Based Payment Processor
## Run Instructions
To run this program, simply run `METHOD_API_KEY={your_key} make up` to spin up the DB (localhost:3306), API (localhost:3001), and UI (localhost:3000).
Note: cargo is takes especially long to build in docker, first run may take 10-15m to compose

## Additional Info
Upon running, you can go to the dashboard page by clicking your icon. There, you will find the Dashboard option. Upon navigating to this page you will be able to upload your XML for parsing as well as view previous reports
Note: the first run will be the most intensive as at that point, no date exists in method for the employee, payee, payor etc. Once those exist in the DB we skip posting them to method

Functionality I wish to add:
* During parsing, store created info in hashmap, such that I can avoid expensive checks to the DB for if a piece of data exist
* Use Diesel crate to avoid repeated SQL work
* Various TODO items spread within the codebase
* Improve User facing experience
* Tell the User what invalid data they send in report
* Unit tests
* End to end tests
