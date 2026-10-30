# performance

性能是使用计算机的关键指标, 目标是对于一个给定cost的计算机获取最好的性能。
计算机系统使用者能够描述他们的性能需求
并且可以从两个可替换的系统选出最适合他们的

我知道的metric = what why how when where who

本书主要描述性能分析的方法和技术, 解决day to day problems

1. 指定性能需求
2. 指导设计
3. 比较多个系统的性能
4. 测量相关指标的值 (system tuning)
5. 找到性能瓶颈 (bottleneck identification)
6. characterizing the load on the system  (workload charaterization) 
7. 测量组建的数量和大小 (capacity planning)
8. 预测未来的负载 (forecasting)

ps: 对象实体是硬件，软件，组建的集合
    

问题描述:

1. 选择针对某个系统合适的测量技术(测量，模拟，建模)，性能指标(响应时间，吞吐量，TPS)，工作负载
2. 引导正确的性能测量 (load generator) && (monitor) 例如: remote terminal emulator
3. 用更适合的统计技术比较, 如果测量多次，每次结果都有些不同，简单的使用结果的平均值可能会导致得到不正确的结论
4. Design measurement and simulation experiments to provide the most information with the least effort.  
5. 正确地性能模拟 
6. 使用简单的排队模型去分析系统的性能

听到的会忘记，看到的会记住，做过的会理解







