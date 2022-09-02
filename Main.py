index_file_name = "enwiki-20211020-pages-articles-multistream-index.txt"

def get_article_offset_id(article_title):
    with open(r"E:\Data" + "\\" + index_file_name, encoding="utf-8") as f:
        lines = 0
        for line in f:
            data = line.split(":")
            title = data[2].strip()
            offset = int(data[0])
            id = int(data[1])

            print(article_title, title)
            if article_title == title:
                print("Found Article")
                return (offset, id)
            input()
        print("Invalid Article")

print(get_article_offset_id("AccessibleComputing"))