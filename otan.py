def Otan():
    try:
        print("Ecrivez le texte à transcrire en OTAN : ")
        array = {' ': ' ', 'A': 'Alpha', 'B': 'Bravo', 'C': 'Charlie', 'D': 'Delta', 'E': 'Echo', 'F': 'Foxtrot',
                'G': 'Golf', "H": "Hotel", 'I': 'India', 'J': 'Juliet', 'K': 'Kilo', 'L': 'Lima', 'M': 'Mike',
                'N': 'November', 'O': 'Oscar', 'P': 'Papa', 'Q': 'Quebec', 'R': 'Romeo', 'S': 'Sierra', 'T': 'Tango',
                'U': 'Uniform', 'V': 'Victor', 'W': 'Whiskey', 'X': 'Xray', 'Y': 'Yankee', 'Z': 'Zulu'}
        otanInput = input('\n').upper()
        if(any(chr.isdigit() for chr in otanInput)):
            return print('\nVeuillez ne pas mettre de nombres.')
        translate = list(map(array.get, otanInput))
        print("\n" + " ".join(translate) + "\n")
    except Exception as err:
        print(f"[ERREUR]: {err}")
Otan()
