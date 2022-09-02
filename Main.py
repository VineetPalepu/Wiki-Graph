import bz2
from distutils.command.config import dump_file
from pkgutil import get_data

index_file_name = "enwiki-20211020-pages-articles-multistream-index.txt"
dump_file_name = "enwiki-20211020-pages-articles-multistream.xml.bz2"
block_size = 262144

def get_article_offset_id(article_title):
    with open(r"E:\Data" + "\\" + index_file_name, encoding="utf-8") as f:
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
    offset, id = get_article_offset_id(article_title)
    with open(r"E:\Data" + "\\" + dump_file_name, mode="rb") as f:
        f.seek(offset)
        d = bz2.BZ2Decompressor()
        # reads the bytes of the entire stream (100 articles) and then decompresses them
        block = f.read(block_size)
        data = d.decompress(block)
        prev_data = data # maybe use for the off chance that the header of the xml gets split in two blocks
        while data.find("<title>{}</title>".format(article_title).encode()) == -1:
            block = f.read(block_size)
            data = d.decompress(block)

        article_title_index = data.find("<title>{}</title>".format(article_title).encode())

        start = article_title_index
        while data.find("</page>".encode(), start) == -1:
            block = f.read(block_size)
            data.append(d.decompress(block))
            start = 0

        article_end_index = data.find("</page>".encode(), article_title_index) + len("</page>".encode())

        article_start_index = data.rfind("<page>".encode(), 0, article_title_index)
        if article_start_index == -1:
            print("was really hoping to not see this message cause the fix is annoying and i'm lazy")
            raise

        article_data = data[article_start_index:article_end_index]

        return article_data
        

# write to file
def wtf(filename, data):
    with open(filename, mode="wb") as f:
        f.write(data)

xml = get_article("Miradz")
wtf("out.xml", xml)