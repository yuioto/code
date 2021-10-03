# ”import os“ 等相关仅原生依耐于 Arch 分支的 Linux 发行版
#初始化依耐项
import os

#用于跨平台的pygame库解决方案
#import pygame
#a = pygame.image.load("0.png")

#打印文字
print("[info] hello,world!")
print("欢迎来到数字翔安。")

print("继续运行需要安装viewnior程序！")        #告知用户请求 sudo 用途
os.system("sudo pacman -Syu viewnior")    #确保有相关依耐

os.system("viewnior /home/wulin/0.png")    #打开图片数字翔安

print("期待下次的见面！")                     #结束语
