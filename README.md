## Rezi

A not so dumb cart helping with grocery lists, managing recipes and meal planning.

### Why

- Reducing mental load - Life is hard, this should make life easier
- Because writing a grocery list should not be this hard.
- Because recipes and grocery lists should be open and accessable

### What can you do?

Ask for a recipe, copy paste in a recipe, or provide a recipe URL - the integrated LLM will intelligently extract structured recipe information and generate grocery lists for you. Choose between quick processing for grocery lists or full structured recipe extraction.

### Features

- **Structured Recipe Extraction**: Automatically parse recipes into organized ingredients, instructions, and metadata
- **Smart Grocery Lists**: Generate and manage shopping lists from recipes or free text
- **Multiple LLM Providers**: Support for both OpenAI and Google Gemini APIs
- **Recipe Management**: Save, organize and search your recipes
- **Export Functionality**: Download recipes in different formats
- **Dual Processing Options**: Quick grocery list generation or full recipe structure extraction

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

#### LLM Providers: OpenAI & Google Gemini

- Supports both OpenAI and Google Gemini APIs for recipe processing
- No persistent data stored by LLM providers
- All data sent is recipe-focused and non-personal
- Choose your preferred provider via configuration

## Configuration

### LLM Setup
Set up your preferred LLM provider by configuring these environment variables:

```bash
# Choose provider: "gemini" or "openai"
LLM_PROVIDER=gemini

# For Gemini:
GEMINI_API_KEY=your-gemini-api-key

# For OpenAI:
OPENAI_API_KEY=your-openai-api-key

# Or use generic key:
LLM_API_KEY=your-api-key
```

### Database & Authentication
- **Database**: Turso.io (Europe-based, trusted provider)
- **Authentication**: Auth0.com (Europe-based, trusted provider)

### Future Plans
I intend to replace third-party vendors with self-hosted alternatives:
- Database: Self-hosted in the same location as the server
- Authentication: Self-hosted solution
- LLM: https://www.infomaniak.com (Swiss company, hosted in Switzerland)
