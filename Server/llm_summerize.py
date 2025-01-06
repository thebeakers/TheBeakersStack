
import json
from pprint import pprint
from dotenv import load_dotenv
from google import genai
import os
import pickle
from pypdf import PdfReader
import requests
from classes import ChemrxivItem

open_pickle_file = open("top_10_items_of_the_week.pkl", "rb")
items = pickle.load(open_pickle_file)

def get_pdf(item: ChemrxivItem):
    if os.path.exists(f"{item.title}.pdf"):
        pass
    else:
        url = item.asset.original.url
        response = requests.get(url)
        with open(f"{item.title}.pdf", "wb") as f:
            f.write(response.content)
    try:
        with open(f"{item.title}.pdf", "rb") as f:
            pdf_reader = PdfReader(f)
            paper = ""
            for page in pdf_reader.pages:
                paper += page.extract_text()
    except FileNotFoundError:
        return "Error: PDF file not found."
    return paper

    

def summerize_pdf(item: ChemrxivItem):
    paper = get_pdf(item)
    return send_to_llm('Explain the following chemistry paper to an undegrade student, write with 5 sentencesw:' + paper + 'in the format of a {response: ""}')


#TODO: Look into thing thing 
# import google.generativeai as genai
# import typing_extensions as typing
# class Recipe(typing.TypedDict):
#     recipe_name: str
#     ingredients: list[str]
# model = genai.GenerativeModel("gemini-1.5-pro-latest")
# result = model.generate_content(
#     "List a few popular cookie recipes.",
#     generation_config=genai.GenerationConfig(
#         response_mime_type="application/json", response_schema=list[Recipe]
#     ),
# )
# print(result)




def generate_questions(item: ChemrxivItem):
    paper = get_pdf(item)
    generate_questions_prompt = """
Generate 10 questions based on the paper and the following course content, the questions should be multiple choice questions with 4 answers, the answers should be in the format of a list of strings, the correct answer should be the first answer in the list, it also must be json format, 
do not mention the course content in the response, do not respond with anything else than the json, it must be serializable by json.loads()
response:


{
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4],
        "question": [answer1, answer2, answer3, answer4]
}


### Course Content List for CHEM101: General Chemistry I

1. **Matter and Measurements**  
2. **The Atom**  
3. **Bonding**  
4. **Chemical Formulas and Equations**  
5. **States of Matter**  
6. **Thermochemistry and Thermodynamics**  
7. **Acid-Base and Oxidation-Reduction Reactions**  
8. **Nuclear Chemistry**
""" + paper
        
    responce = send_to_llm(generate_questions_prompt)

    import re
    json_pattern = re.compile(r'\{.*\}', re.DOTALL)
    match = json_pattern.search(responce)
    if match:
        json_string = match.group(0)
        try:
            questions = json.loads(json_string)
            return questions
        except json.JSONDecodeError as e:
            print(f"Error decoding JSON: {e}")
    else:
        print("No JSON found in the response.")



def send_to_llm(context):
    load_dotenv(dotenv_path = ".env")
    api_key = os.getenv("GEMINI_API_KEY")
    if api_key:
        client = genai.Client(api_key=api_key)
        response = client.models.generate_content(model='gemini-2.0-flash-exp', contents=context)
        return response.text
    else:
        print("Error: GEMINI_API_KEY not found in .env file.")



print(summerize_pdf(items[0]))
print(json.dumps(generate_questions(items[0]), indent=4))
