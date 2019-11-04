# substrate-post

石墨文档链接：
https://shimo.im/docs/hYrdrWDXjqQkjyKq/ 《Substrate面试题》，可复制链接后用石墨文档 App 或小程序打开


substrate post parcel

Goal

Build a Substrate based blockchain to support courier post tracking functionality.
Requirements
Managers can add / remove operators.
Managers can add / remove managers.
Operator can create a parcel.
A parcel have ID, source, destination, receiver, coordinates and status.
Operator can add new coordinates to a parcel.
Receiver can sign-off a parcel and change the status to received.
Note: Please make sure your code is high quality and ready for production. Also try to keep it as simple as possible while fulfilling all the requirements.

## 分析

### 1- 角色
0- 管理员（Manage）
1- 操作者（Operator） - 快递员
2- 接收人（Receiver）- 接收人

### 2- 事件

A-系统账号
1- 创建管理员
2-删除管理员
3-根据AccountID查询创建的Manager列表-LinkItem实现：TODO

B-管理员账号
1-创建操作者
2-删除操作者
3-根据Manager查询创建的OPerator列表-LinkItem实现：TODO

C-操作者账号
1- 创建包裹
2-更新包裹位置
3-删除包裹：TODO
4-根据OperatorId查询创建的Parcel列表-LinkItem实现

D-接收人
1- 更新包裹状态-Received

### 功能实现

#### 1- post_role
创建和删除Manager以及Operator

decl_storage!
        ///-----------------------Manager-----------------------
        pub ManagerCount get(manager_count): T::RoleIndex;

        pub Managers get(managers): map T::RoleIndex => Option<T::AccountId>;
        pub ManagerToIndex get(manager_to_index): map T::AccountId => Option<T::RoleIndex>;
        
        pub CreatorMangerList get(creator_manager_list): map (T::AccountId, T::AccountId) => Option<T::RoleIndex>;
///-----------------------operator-----------------------
        pub OperatorCount get(operator_count): T::RoleIndex;
        
        pub Operators get(operators): map T::RoleIndex => Option<T::AccountId>;
        // pub ManagerOwners get(manager_owner): map T::RoleIndex => Option<T::AccountId>;
        
        pub OperatorToIndex get(operator_to_index): map T::AccountId => Option<T::RoleIndex>;
        
        pub ManagerOperatorList get(manager_operator_list): map (T::AccountId, T::AccountId) => Option<T::RoleIndex>;
 
decl_module!

    pub fn add_post_manage_list(origin, account_id:T::AccountId) ->Result
    
    pub fn remove_manage_list(origin, account_id:T::AccountId) ->Result {
 
    pub fn add_post_operator_list(origin, account_id:T::AccountId) ->Result {
    
    pub fn remove_post_operator_list(origin, account_id:T::AccountId) ->Result {
 
#### 2-post_parcel
1- 创建包裹
2-更新包裹位置
 
decl_storage!
 
        /// Stores the total number of parcel. i.e. the next kitty index
        pub ParcelCount get(parcel_count): T::ParcelIndex;
        
        /// Stores all the Parcel, key is the parcel id/index
        pub Parcels get(parcels): map T::ParcelIndex => Option<Parcel<T>>;
        
        /// the operator owned the parcels
        pub OwnedParcels get(owned_parcels): map (T::AccountId, Option<T::ParcelIndex>) => Option<ParcelLinkedItem<T>>;
        
        /// Get Parcels owner
        pub ParcelOwners get(parcel_owners): map T::ParcelIndex => Option<T::AccountId>;
        
        /// Get Parcels owner
        pub ParcelReceiver get(parcel_receiver): map T::ParcelIndex => Option<T::AccountId>;
        
        /// creator and the manager list
        pub OperatorParcelList get(operator_parcel_list): map (T::AccountId, T::AccountId) => Option<T::ParcelIndex>;
 
decl_module!
 
    pub fn create_parcel(origin, _source: Vec<u32>, _destination: Vec<u32>, _receiver: T::AccountId, _coordinates: (i16,i16), _state: u8) -> Result {
 
    pub fn _update_parcel_coordinates(origin, parcel_index: T::ParcelIndex, _coordinates_params: (i16,i16)) -> Result {
 
#### 3-post_receiver
1- 更新包裹状态-Received
 
    pub fn update_parcel_state_to_received(origin, _parcel_index: T::ParcelIndex, _state_params: u8) -> Result {
 

### 问题：
在做这个项目的过程中发现了如下的问题
1- 跨rs文件Module以及Trait的引1
2- 页面JS异常，因为参数配置错误；并且也尝试了本地启动UI的方式，没有成功。
3- 参数和类型的使用

### 流程：
1- 在做项目之前，又从新学习了一遍Substrate的课程，
2- 对项目的需求分析用了大概1天左右的时间，需求很明确，总感觉有一些地方没想明白。
3- 用了2天左右的时间，进行编码，其中遇到了一些异常的问题，用了一些时间，在编码的过程中对之前的代码又做了重构；
4- 参考项目Hello Kitty项目
 
### 后续：
1-顺利流程和程序中隐藏的Bug：比如State状态更新不成功。
2-测试代码添加- 目前通过JS完成测试。
2-TODO的功能没有完成。
3-是否需要重构，或者怎么划分的更加合理，一直在考虑。
测试截图：
0-创建测试账号
https://uploader.shimo.im/f/pDoQMX5fdiwc74ir.png!thumbnail

1- Manager
添加Manger001
 https://uploader.shimo.im/f/XoNJddByWYA1Kk01.png!thumbnail
 
添加Manger002
 https://uploader.shimo.im/f/VtboMoXwsW45fRs5.png!thumbnail
 
Storage内容查看
 https://uploader.shimo.im/f/rcUDOkd6k4MqyBSZ.png!thumbnail


2-Operator
用Manager001测试
添加Operator001
 https://uploader.shimo.im/f/qyyJoR7CiVgm1Hji.png!thumbnail
 
添加Operator002
 https://uploader.shimo.im/f/aWVj1e4mbdEOptAI.png!thumbnail
 
Storage查看
https://uploader.shimo.im/f/RBatWWG54Z4xRSzg.png!thumbnail
 
3-Parcel
用Operator001测试
 
 https://uploader.shimo.im/f/dirunvJuCl0wWf38.png!thumbnail
 
 https://uploader.shimo.im/f/gNpZdrKj8EcXYstS.png!thumbnail

 === 数据结构使用的不合理，造成数据不正确。

4-Receiver
 
 
JS参数设定
{
  "RoleIndex": "u32",
  "ParcelIndex": "u32",
  "ParcelStruct": {
    "p_id": "ParcelIndex",
    "p_receiver": "u64",
    "p_state": "u32",
    "p_source": "[u8; 16]",
    "p_destination": "[u8; 16]",
    "p_coordinates": "Vec<(i16,i16)>"
  },
  "ParcelLinkedItem": {
    "prev": "Option<ParcelIndex>",
    "next": "Option<ParcelIndex>"
  }
}