import os

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
DICT_PATH = os.path.join(BASE_DIR, "Dictionnarys", "Dictionnary.txt")

def createDictionnary():
    DICTIONARY = []

    
    with open(DICT_PATH, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            word = line.split()[0]
            DICTIONARY.append(word)

    print("Dictionnary created")
    return DICTIONARY