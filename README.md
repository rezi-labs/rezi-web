## Rezi

A not so dumb cart helping with grocery lists, managing recipes and meal planning.

### Why

- Reducing mental load - Life is hard, this should make life easier
- Because writing a grocery list should not be this hard.
- Because recipes and grocery lists should be open and accessable

### What can you do?

Ask for a recipe, copy paste in a recipe, ask for a certain kind of thing you want to buy and the magic of LLMs will create the items for you.

### Other

- save recipes
- talk to Rezi to find new recipes
- download recipes in different format

### Important for You

The way this application is written is very intentional.
What does that mean:

- As fast as it makes sense
- Responsible resource usage
- Privacy whenever possible
- You own your data

How are those things achieved:

#### As fast as it makes sense

I did write it in programming languages and concepts that can run without need a lot of computing resources

#### Responsible resource usage

- Read again: As fast as it makes sense
- I do not dump a lot to the LLM, this in particular means that by all means I try to minimize asking the LLM and when I do, I reduce the size of the prompt and possible output to a minimum.

#### Privacy whenever possible

- Whenever the LLM, they can not see any personal data if you did not put that data inside the chat. This also means that the LLM vendor will not know which message belongs to whom.

#### You own your data

- The goal is to always allow you to download the messages, recipes and grocery list in different formats.

#### Thirdparty Vendors

#### Authentication: auth0.com

- This means they will have your email adress, they are not allowed to use it in any means.
- No spam mails etc.
- The are a trusted company by many big and small companies around the world.
- The data lies in Europe

#### Database: Turso.io

- This company will have all the data you input into the application.
- This company is also highly trusted
- The data lies in Europe

#### LLM: Google

- This Company is not trustworthy with any personal data
- There is no data persistent for this provider
- All the data being sent is non personal this means: the data can not be used to correlate your inputs with your person. The exception is if you write and send in the chat your contact details for every message.

I intend to remove or replace those vendors in the near future.

- Database: selfhosted - the same place where this server is hosted
- Authentication: selfhosted - the same place where this server is hosted
- LLM: https://www.infomaniak.com - swiss company - all hostend in switzerland
