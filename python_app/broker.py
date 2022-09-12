#!/usr/bin/env python
import os
import json
import torch
import logging
from logging.config import fileConfig
from torchvision.models import vgg11
import albumentations as A
from albumentations.pytorch import ToTensorV2
from juans.utils.image import read_image


class Model:
    def __init__(self, **kwargs):
        # FIXME: When rust call python, the __file__ is null
        if kwargs.get('app_path'):
            log_conf = os.path.join(kwargs['app_path'], 'logging.conf')
            self.demo_image = os.path.join(kwargs['app_path'], 'test.jpg')
        else:
            log_conf = 'logging.conf'
            self.demo_image = 'test.jpg'
        fileConfig(log_conf)
        self.logger = logging.getLogger('file')
        self.test_transforms = A.Compose([A.Resize(32, 32), A.Normalize(), ToTensorV2()])
        self.model = vgg11()
        self.model.eval()
        self.logger.info('finish init.')

    def predict(self, **kwargs):
        image_data = read_image(self.demo_image)
        one_batch = self.test_transforms(image=image_data)["image"].unsqueeze(0)
        with torch.no_grad():
            model_output = self.model(one_batch).squeeze().argmax()
        return json.dumps({
            "output": model_output.item()
        })
