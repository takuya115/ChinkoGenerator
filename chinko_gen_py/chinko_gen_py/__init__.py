from chinko_generator import ChinkoGenerator

def main():
    chinko_gen = ChinkoGenerator()
    while not chinko_gen.gen_triplet():
        pass

    print(f"{str(chinko_gen)}(ﾎﾞﾛﾝｯ")
    print(f"{len(str(chinko_gen))}文字目で出ました")

if __name__ == "__main__":
    main()
