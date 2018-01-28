import os
import math
import sys
import keras
import numpy as np
from PIL import Image

model_path = os.path.join(os.getcwd(), "assets/doodle-model.h5")
model = keras.models.load_model(model_path)
TARGET_DIM = 28 # target image dimension for the neural network

CATEGORIES = [
    "Baseball",
    "Bowtie",
    "Clock",
    "Hand",
    "Hat",
    "Lightning",
    "Lollipop",
    "Mountain",
    "Pizza",
    "Star"
]

def load_image(path):
    # TODO load as black and white (0 or 1)
    img = Image.open(path) #Can be many different formats.
    img.load()
    print(f'loaded image of size {img.size}')
    print(f'image trasnparency status {img.getbbox()}')
    return img

def trim_image(img):
    img=img.crop(img.getbbox())
    # pad to square
    longer_side = max(img.size)
    horizontal_padding = (longer_side - img.size[0]) / 2
    vertical_padding = (longer_side - img.size[1]) / 2
    img = img.crop(
        (
            -math.ceil(horizontal_padding),
            -math.ceil(vertical_padding),
            img.size[0] + math.floor(horizontal_padding),
            img.size[1] + math.floor(vertical_padding)
        )
    )
    return img

# convert transparent pixels to white
def rasterize(img):
    canvas = Image.new('RGBA', img.size, (255,255,255,255)) # Empty canvas colour (r,g,b,a)
    canvas.paste(img, mask=img) # Paste the image onto the canvas, using it's alpha channel as mask
    return canvas

def resize_image(image):
    assert image.size[0] == image.size[1], f"image should be square {image.size}"
    wpercent = (TARGET_DIM/float(image.size[0]))
    hsize = int((float(image.size[1])*float(wpercent)))
    image = image.resize((TARGET_DIM,hsize), Image.ANTIALIAS)
    return image

def save_image(img, path):
    img.save(path)

# convert grayscale to black and white
def black_white(pixels):
    pixels[pixels < 255] = 0
    return pixels

def inverse(pixels):
    pixels *= -1
    pixels += 255
    return pixels


if __name__ == '__main__':
    if (len(sys.argv) < 2):
        print('pass in an image')
        sys.exit()
    path = sys.argv[1]
    image = load_image(path)
    image = trim_image(image)
    image = rasterize(image)
    image = resize_image(image)
    image = image.convert('L') # make it grayscale
    save_image(image, 'cropped_drawing.png')

    image_data = np.asarray(image, np.float32)
    image_data = image_data.reshape(28*28) # redundant?
    image_data = inverse(image_data)

    image_data = image_data.reshape(1, 28, 28, 1)
    image_data /= 255 # 
    print('image shape', image_data.shape)

    probs = model.predict(image_data)[0]
    predictions = dict(zip(CATEGORIES, probs))
    predictions = sorted(predictions.items(), key=lambda x:x[1], reverse=True)
    best_guess = predictions[0]
    print(predictions)
    print(f'{best_guess[0]},{best_guess[1]}')

