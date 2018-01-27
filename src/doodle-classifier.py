import os
import sys
# import keras
import numpy as np
from PIL import Image

model_path = os.path.join(os.getcwd(), "assets/doodle-model.h5")
# model = keras.models.load_model(model_path)
TARGET_DIM = 28 # target image dimension for the neural network


def predict(imageData):
    img = np.array(imageData, np.float32).reshape(1, 28, 28, 1)
    img /= 255
    result = model.predict(img)[0]
    return result

def load_image(path):
    img = Image.open(path) #Can be many different formats.
    pix = img.load()
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
            -horizontal_padding,
            -vertical_padding,
            img.size[0] + horizontal_padding,
            img.size[1] + vertical_padding
        )
    )
    return img


def resize_image(image):
    assert image.size[0] == image.size[1], "pass in a square image"
    wpercent = (TARGET_DIM/float(image.size[0]))
    hsize = int((float(image.size[1])*float(wpercent)))
    image = image.resize((TARGET_DIM,hsize), Image.ANTIALIAS)
    return image

def save_image(img, path):
    img.save(path)


if __name__ == '__main__':
    if (len(sys.argv) < 2):
        print('pass in an image')
        sys.exit()
    path = sys.argv[1]
    image = load_image(path)
    image = trim_image(image)
    # image = resize_image(image)
    # save_image(image, 'cropped_drawing.png')
    print(image.getpixel((0,0)))

# TODO pixelval /= 255

    # print(predict(imagedata))
