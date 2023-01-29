# FOLIO utilities

DISCLAIMERS: This collection of lightweight self-contained utilities comes without any warranty or support. Used improperly, they can do enormous damage to FOLIO systems, so don't assume any individual function is safe for your purposes -- it may have been created to meet a different need. They assume you know what you're doing, and won't warn you about  prerequisites, gotchas, etc. to using them. Some scripts allow you to do things FOLIO is not designed to support. Please be prudent and considerate so everyone gets the best experience and the FOLIO API remains as unlimited as possible.

The FOLIO *learning-apis* Slack channel and other forums are great places to ask questions and share insights.

# Use

Most scripts here require the jq utility to use and have been tested only on WSL and Amazon Linux. Virtually all assume you have the following files in your working directory:

- *tenant* -- contains the ID of your FOLIO tenant
- *okapi.url* -- contains the okapi URL for your tenant
- *okapi.token* -- contains a valid okapi token

To get an okapi token, you will first need to run the *auth* script. The *auth* script requires one additional file called *okapi-login.json* found in this directory which can be modified for your credentials.

Any API requiring an edge key will also require a file named *edge.key*

API operations are allowed based on the permissions assigned to the user. If all the permissions you've assigned via the interface are insufficient, you'll need to retrieve your user via the API, generate a token using your administrative users and run the permission-add script.

In general, scripts should accept logical inputs for worflows that would call for their use -- for example,*item-get* does not care if you supply a barcode, HRID, or UUID since these are all logical inputs. *uuid-lookup* doesn't care if you're trying to find codes or names based on UUID or the other way around. *oai-harvest* with no arguments assumes you're checking to see if a record that was just modified gets output via OAI, but if it sees a date or UUID as an argument, it issues an incremental harvest from the date supplied or a GetRecord directive respectively. The best way to figure out what assumptions a script makes is to just look at it -- all of them are short.

It's assumed that the scripts will be your path so they can simply be called, though some require modification for the specifics at hand. In scripts that use threads, it is recommended that you don't increase the number of threads unless your institution has its own cluster -- the values you see were determined experimentally to be the maximum safely useable on most shared tenants and need to account for regular use. The safe level may change as more libraries use the API themselves.

Replication of function reflects how common some operations are. *records-get locations* and *locations-get* achieve the same effect but reflect how often all locations are retrieved. 

