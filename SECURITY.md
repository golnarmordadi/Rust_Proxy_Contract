# Security Policy

Please see below for the security policy.

## Reporting a Vulnerability

Please report any security issues via email to `golnar@soarrobotics.com`.

You will receive a response from us within 4 working days confirming that a human read your email. If you do not hear back within 1 week, feel free to send a reminder or try to notify core team members via different channels.

Within a few days we try to reproduce the issue and confirm it. After that we work on a patch and a release strategy. Experience shows the later part is harder than the actual patch as we need to evaluate which versions are affected, for which versions a patch is provided, if that patch is consensus or state breaking and how users can apply the patch. This part can take a few days up to multiple weeks.

Please avoid opening public issues on GitHub that contains information about a potential security vulnerability as this makes it difficult to reduce the impact and harm of valid security issues.

## Coordinated Vulnerability Disclosure Policy

We ask security researchers to keep vulnerabilities and communications around vulnerability submissions private and confidential until a patch is developed. In addition to this, we ask that you:

Allow us a reasonable amount of time to correct or address security vulnerabilities.
Avoid exploiting any vulnerabilities that you discover.
Demonstrate good faith by not disrupting or degrading services built on top of this software.

## Vulnerability Disclosure Process

We are using the following disclosure process for the repo:

* Once a security report is received, the core development team works to verify the issue.
* Patches are prepared for eligible releases in private repositories.
* We notify the community that a security release is coming, to give users time to prepare their systems for the update.
* The fixes are applied publicly and new releases are issued.
* Once releases are available, we notify the community, again, through the same channels as above.
* Once the patches have been properly rolled out and no earlier than 7 days after the release, we will publish a post with further details on the vulnerability as well as our response to it.
* Note that we are working on a concept for bug bounties and they are not currently available.
* This process can take some time. Every effort will be made to handle the bug as quickly and thoroughly as possible. However, it's important that we follow the process described above to ensure that disclosures are handled consistently and to keep this codebase and the projects that depend on them secure.

## Exceptions

Please note that issues are considered already disclosed if there is a public patch for it or it is discussed in public. In those cases the above process does not apply. An exception can be cases where the fix was well hidden and neither the fix not the public discussion reveals a vulnerability was patched. Advisories are created and published in any case for future reference, giving credit to the people involved and the learning opportunity.
