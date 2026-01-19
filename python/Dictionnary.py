import os

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
#DICT_PATH = os.path.join(BASE_DIR, "dictionarys", "dictionary.txt")
#DICT_PATH = os.path.join(BASE_DIR, "dictionarys", "dictionary_westeros.txt")
#DICT_PATH = os.path.join(BASE_DIR, "dictionarys", "dictionary_star_wars.txt")
#DICT_PATH = os.path.join(BASE_DIR, "dictionarys", "dictionary_star_trek.txt")
DICT_PATH = os.path.join(BASE_DIR, "dictionarys", "dictionary_harry_potter.txt")

def createdictionary():
    DICTIONARY = []
    
    with open(DICT_PATH, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            word = line.split()[0]
            DICTIONARY.append(word)

    print("dictionary created")
    return DICTIONARY