# FOLIO utilities

DISCLAIMERS: This collection of lightweight self-contained utilities comes without any warranty or support. Used improperly, they can do enormous damage to FOLIO systems, so don't assume that because a function exists that it's safe -- it could have been designed for a need different than yours. They assume you know what you're doing, so if you need to run other processes to use a script safely, they won't tell you. Some of these scripts allow you to do things FOLIO is simply not designed to support. Please be considerate and don't use them inappropriately so FOLIO stays as open as possible.

The FOLIO *learning-apis* Slack channel and other appropriate forums are great places to ask questions and share insights.

Most scripts here require the jq utility to use. These have been tested only on WSL and Amazon Linux. Virtually all assume you have the following files in your working directory:

- *tenant* -- contains the ID of your FOLIO tenant
- *okapi.url* -- contains the okapi URL for your tenant
- *okapi.token* -- contains a valid okapi token

To get an okapi token, you will first need to run the *auth* script. The *auth* script requires one additional file called *okapi-login.json* found in this directory which can be modified for your credentials.

Any API requiring an edge key will also require a file named *edge.key*

API operations are allowed based on the permissions assigned to the user. If all the permissions you've assigned via the interface are insufficient, you'll need to retrieve your user via the API, generate a token using your administrative users and run the permission-add script.

# Use

In general, scripts should accept logical inputs for their respective workflows -- for example,*item-get* does not care if you supply a barcode, HRID, or UUID since these are all logical inputs. *uuid-lookup* doesn't care if you're trying to find codes or names based on UUID or the other way around. *oai-harvest* with no arguments assumes you're checking to see if a record that was just modified gets output via OAI, but if it sees a date or UUID as an argument, it will presume an incremental harvest from the date supplied or issue a GetRecord directive. 
 
It's assumed that the scripts will be your path so they can simply be called, though some require modification for the specifics at hand. In scripts that use threads, it is recommended that you don't increase the number of threads unless you have specific reason to believe it is safe -- the values you see were determined experimentally to be the maximum safely useable on most shared tenants and need to account for ongoing use. 

Replication of function reflects how common some operations are. *records-get locations* and *locations-get* achieve the same effect but reflect how often all locations are retrieved. 

