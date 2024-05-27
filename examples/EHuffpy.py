import huffpy

# A basic example of how huffpy work

text = "This is a sample text"

hfp = huffpy.Huffpy() # Create a new instance of Huffpy Class
hfp.Create(text) # Create a new huffpy tree
res_encoded_str = hfp.Encode() # Encode the text and return the encoded string
res_decoded_str = hfp.Decode(res_encoded_str) # Decode the encoded string and return it

print("Original Text:", text);
print("Encoded Text:", res_encoded_str);
print("Decoded Text:", res_decoded_str);
