import random

class ChinkoGenerator:
    _is_chinko = [0, 1, 2]
    _chinko_dict = ["ち", "ん", "こ"]

    def __init__(self) -> None:
        self._chinko_list = []

    def __str__(self) -> str:
        word = [self._chinko_dict[int(elm)] for elm in self._chinko_list]
        return "".join(word)
    
    def gen_triplet(self):
        triplet = [random.randint(0, 2) for _ in range(3)]
        self._chinko_list.extend(triplet)
        return triplet == self._is_chinko
    

            
