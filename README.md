<img width="1222" height="732" alt="image" src="https://github.com/user-attachments/assets/975e350b-1a94-4332-9851-5c283f05a8d2" />

# How to Use
1. The UI is divided into separately scrolling lists of characters, weapons, artifact sets and TCG cards. Each list has a separate search bar.
2. Enter the IDs of the items you wish to query in the search bar at the bottom of the page and press "Search".
3. The items will be queried using the Hakush.in website's API endpoints.
4. The responses will be reduced to the key details, formatted for clearness and saved as json files in the results folder (which contains separate folders for each type of item).
5. When querying something more than once, the originally saved file will also be opened to compare the two versions. If differences are found, the new data replaces the old and a file will be generated in "changes/{item type}/{item name} {date}.json".
6. Python code is used to compare the two item structs as I find its DeepDiff package much simpler to understand as an end user. If this fails, the program will default to a Rust-native alternative.

# To Dos
1. Update the lists in the UI to feature checkboxes for faster searching. This way the user will not need to manually enter every ID they want to look up (though the manual entry element will be retained for those who wish to).
2. Add functionality for a shortlist.txt file to be handled. Users can add IDs they plan to look up frequently here ahead of time and call them automatically.
