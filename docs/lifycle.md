# Rust 生命周期
简短回答：生命周期代表资源的可用时期。显式生命周期标记是泛型参数的一种，但是有协变的概念。长生命周期作为短生命周期的子类型，此处与直觉相反。摘自 [生榴莲](https://www.zhihu.com/question/48196609)

declare a variable, and the life cycle will start
