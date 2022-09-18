import bz2
import timeit

path = "C:\\Users\\Vineet Palepu\\Downloads\\enwiki-20220101-pages-articles-multistream\\"
index_file_name = "enwiki-20220101-pages-articles-multistream-index.txt"
dump_file_name = "enwiki-20220101-pages-articles-multistream.xml.bz2"
block_size = 262144

def get_article_offset_id(article_title):
    with open(path + index_file_name, encoding="utf-8") as f:
        lines = 0
        for line in f:
            data = line.split(":")
            title = data[2].strip()
            offset = int(data[0])
            id = int(data[1])
            if article_title == title:
                print("Found Article:", offset, id)
                return (offset, id)
        print("Invalid Article")
        raise

def get_article(article_title):
    t0 = timeit.default_timer()
    print("Searching index for article")
    offset, id = get_article_offset_id(article_title)
    t1 = timeit.default_timer()
    print("{} seconds to search index".format(t1-t0))
    with open(path + dump_file_name, mode="rb") as f:
        f.seek(offset)
        d = bz2.BZ2Decompressor()
        # reads the bytes of the entire stream (100 articles) and then decompresses them
        block = f.read(block_size)
        data = d.decompress(block)
        prev_data = data # maybe use for the off chance that the header of the xml gets split in two blocks
        while data.find("<title>{}</title>".format(article_title).encode()) == -1:
            block = f.read(block_size   )
            data = d.decompress(block)

        article_title_index = data.find("<title>{}</title>".format(article_title).encode())

        start = article_title_index
        while data.find("</page>".encode(), start) == -1:
            print("------------multi page article------- this code actually did something")
            input()
            block = f.read(block_size)
            data.append(d.decompress(block))
            start = 0
        article_end_index = data.find("</page>".encode(), article_title_index) + len("</page>".encode())

        article_start_index = data.rfind("<page>".encode(), 0, article_title_index)
        if article_start_index == -1:
            print("was really hoping to not see this message cause the fix is annoying and i'm lazy")
            raise

        article_data = data[article_start_index:article_end_index]



        t2 = timeit.default_timer()
        print("{} seconds to find article".format(t2 - t1))
        print("{} total seconds elapsed".format(t2 - t0))

        return article_data
        

# write to file

def wtf(filename, data):
    with open(filename, mode="wb") as f:
        f.write(data)

xml = get_article("OpenHistoricalMap")
wtf("out.xml", xml)


##### use continuous integration / continuous development or some shit like that to implement automatic performance testing