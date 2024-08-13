import string


class CaesarCypher:
    def __init__(self, shift_amt: int):
        self.ref = self._create_ref()
        self.shifted_ref = self._shift_ref(shift_amt)
        self.shift_amt = shift_amt

    @staticmethod
    def _create_ref() -> dict:  # Create reference list
        ind: int = 1  # Start index for ref
        ref: dict = {}  # Reference dict
        for x, y in zip(string.ascii_uppercase, string.ascii_lowercase):
            ref[ind] = [x, y]
            ind += 1

        return ref

    def _shift_ref(self, shift_amt: int) -> dict:  # Create shifted list
        shifted_ref: dict = {}
        for x, y in self.ref.items():
            shifted_ref[self._inside_range(x + shift_amt)] = y

        return shifted_ref

    @staticmethod
    def _inside_range(index: int) -> int:
        if index > 26:
            while index > 26:
                index -= 26
        elif index < 0:
            while index < 0:
                index += 26
        return index

    def _operation(self, phrase: str, reference: dict) -> str:
        result = ""
        for a in phrase:
            if a in string.ascii_uppercase or a in string.ascii_lowercase:
                for x, y in reference.items():
                    if a in y:
                        if a in string.ascii_uppercase:
                            result += reference[self._inside_range(x - self.shift_amt)][0]
                        elif a in string.ascii_lowercase:
                            result += reference[self._inside_range(x - self.shift_amt)][1]
            else:
                result += a

        return result

    def cypher(self, phrase: str) -> str:
        return self._operation(phrase, self.ref)

    def decypher(self, phrase: str) -> str:
        return self._operation(phrase, self.shifted_ref)
