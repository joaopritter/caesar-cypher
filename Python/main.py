from src import CaesarCypher


if __name__ == '__main__':
    op = False
    while not op:
        try:
            input_op = input("Choose 'cypher or 'decypher': ")
            assert input_op in ["cypher", "decypher"]
            op = True
        except AssertionError:
            print("Try Again.")
    input_phrase = input("Enter Phrase: ")
    shift = False
    while not shift:
        try:
            input_shift_amt = int(input("Enter shift amount: "))
            shift = True
        except ValueError:
            print("Try Again.")
    if input_op == "cypher":
        print(CaesarCypher(input_shift_amt).cypher(input_phrase))
    else:
        print(CaesarCypher(-input_shift_amt).decypher(input_phrase))
