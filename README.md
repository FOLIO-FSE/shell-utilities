# FOLIO utilities

A collection of lightweight utilities to interact with FOLIO using cURL. Virtually all assume you have the following files in your working directory:

- *tenant* -- contains the ID of your FOLIO tenant
- *okapi.url* -- contains the okapi URL for your tenant
- *okapi.token* -- contains a valid okapi token

To get an okapi token, you will first need to run the *auth* script. The *auth* script requires one additional file called *okapi-login.json* found in this directory which can be modified for your credentials.

API operations are allowed based on the permissions assigned to the user, so you'll need to make sure you've assigned yourself what you need. If all the permissions you've assigned via the interface are insufficient, you'll need to retrieve your user via the API, generate a token using your administrative users and run the permission-add script.


Some scripts here require the jq utility to use.
