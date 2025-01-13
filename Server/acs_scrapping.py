import os
import pickle
from classes import *


if __name__ == "__main__":
    if os.path.exists("top_10_items_of_the_week.pkl"):
        with open("top_10_items_of_the_week.pkl", "rb") as f:
            x = pickle.load(f)
    else:
        x = get_top_10_items_of_the_week()
        with open("top_10_items_of_the_week.pkl", "wb") as f:
            pickle.dump(x, f)



for item in x:
    print(item.title)   
    print(item.authors)
    print(item.metrics)
    print(item.asset.original.url)
    print("\n")
export_to_frontend(x[0])
