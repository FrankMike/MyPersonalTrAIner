# MyPersonalTrAIner

MyPersonalTrAIner is an automated system that, according to the user physical characteristics and the help of modern LLMs (Large Language Models) such as OpenAI ChatGPT or Anthropic Claude, helps to achieve fitness goals as the user requests.


## Fitness Goals

Fitness goals can vary depending on the individual's needs.

- Weight management:
    - Loosing body fat
    - Gaining muscle mass (bulking)
    - Maintaining current weight
- Strength improvement:
    - Increasing overall strength
    - Focusing on specific lifts (improving bench press, squat or others)
- Endurance enhancement:
    - Improving cardiovascular fitness
    - Preparing for a specific event (marathon, triathlon, etc.)
- Flexibility and mobility:
    - Increasing overall flexibility
    - Improving range of motion in specific joints
- Sport-specific performance:
    - Enhancing skills for a particular sport
    - Improving sport-specific conditioning
- Health-related goals:
    - Lowering blood pressure or cholesterol
    - Managing conditions like diabetes or arthritis
    - Improving bone density
- Functional fitness:
    - Enhancing daily life activities
    - Improving balance and coordination
- Body composition changes:
    - Reducing body fat percentage
    - Increasing lean muscle mass
- Mental health and well-being:
    - Reducing stress through exercise
    - Improving mood and energy levels
- Lifestyles changes:
    - Establishing a consistent exercise routine
    - Adopting healthier eating habits

All these goals can combined for a more beneficial result. For instance, is possible to have these mixed goals:

- Lose 5 kg of body fat
- Increase the squat strength by 20%
- Improve the flexibility to touch toes comfortably
- Run a 5 Km race in under 30 minutes

## Fitness Plan

Here a list of informations needed to create a tailored fitness plan:

1. Age
2. Height
3. Weight
4. Gender
5. Current finess level
    - Beginner: New to regular exercise or returning after a long break
    - Intermediate: Exercise regularly but looking to improve
    - Adanced: Very fit and looking for more challenging routines


    Or a sentence like: "I can jog for 15 minutes without stopping, do 10 push-ups, and I yoga once a week."
6. Any physical limitations or health conditions
7. Time available for exercise each week
8. Equipment or facilities available (home equipments, gym)

### Exercises

For each body part there is a different exercise, in some cases exercises work multiple body parts simultaneously. This is a list that includes a mix of bodyweight exercises and those that may require equipment:

1. Chest:
    - Push-ups
    - Bench press
    - Chest flyes
    - Dips
2. Back:
    - Pull-ups
    - Rows (bent-over, seated, or machine)
    - Lat pulldowns
    - Superman holds
3. Shoulders:
    - Overhead press
    - Lateral raises
    - Front raises
    - Shrugs
4. Biceps:
    - Bicep curls
    - Hammer curls
    - Chin-ups
5. Triceps:
    - Tricep dips
    - Tricep pushdowns
    - Skull crushers
6. Legs:
    - Squats
    - Lunges
    - Deadlifts
    - Leg press
    - Calf raises
7. Core:
    - Planks
    - Crunches
    - Russian twists
    - Leg raises
8. Glutes:
    - Glute bridges
    - Hip thrusts
    - Step-ups
9. Cardiovascular exercises:
    - Running/jogging
    - Cycling
    - Swimming
    - Jump rope
    - Burpees
10. Full body:
    - Mountain climbers
    - Jumping jacks
    - Kettlebell swings


## Install Ollama (on macOS or Linux)
curl https://ollama.ai/install.sh | sh

## Pull a model (e.g., Llama 2)
ollama pull llama2

## Use environment varibles
Fox example: 
OLLAMA_API_ENDPOINT="http://localhost:11434/api/generate"
OLLAMA_MODEL="llama3.2"
