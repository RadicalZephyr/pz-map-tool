import os
from pathlib import Path

save_dirs = (
    Path("Saves/Multiplayer/VanillaBestSettings"),
    Path("Saves/Multiplayer/VanillaBestSettings/isoregiondata"),
    Path("Saves/Multiplayer/VanillaBestSettings_player"),
)

chunk_range = (range(21, 23), range(17, 18))
cell_range = (range(6596, 6866), range(5286, 5568))


def main():
    for dir in save_dirs:
        delete_files(dir)
        print("deleted files in {}".format(dir))


def delete_files(dir):
    prefixes = {"map": cell_range, "chunkdata": chunk_range, "zpop": chunk_range}

    for f in os.listdir(dir):
        f_splits = f.split(".")[0].split("_")
        prefix = f_splits[0]
        if prefix in prefixes:
            (x_range, y_range) = prefixes[prefix]
            if len(f_splits) == 3:
                x = int(f_splits[1])
                y = int(f_splits[2])
                if not (x in x_range and y in y_range):
                    print("REMOVE", dir,f)
                    # os.remove(os.path.join(dir, f))


if __name__ == "__main__":
    main()
