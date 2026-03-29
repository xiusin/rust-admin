export interface TableColumn {
  field: string;
  title: string;
  width: number | "auto";
  minWidth: number | null;
  align: "left" | "center" | "right";
  fixed: "left" | "right" | null;
  sortable: boolean;
  visible: boolean;
  sort: number;
  formatter: string | null;
  render: string | null;
}

export interface TableSearchField {
  field: string;
  title: string;
  searchType: "like" | "eq" | "ne" | "gt" | "gte" | "lt" | "lte" | "in" | "between" | "date" | "daterange";
  fieldType: "text" | "select" | "date" | "daterange" | "cascader" | "tree-select";
  options: { label: string; value: string | number }[] | null;
  placeholder: string | null;
  defaultValue: string | number | null;
  weight: number;
}

export interface TableSearchConfig {
  enabled: boolean;
  fields: TableSearchField[];
  placeholder: string;
  showCollapse: boolean;
  defaultCollapsed: boolean;
}

export interface TableToolbarButton {
  key: string;
  name: string;
  icon: string;
  type: "primary" | "success" | "warning" | "danger" | "info" | "default";
  action: "create" | "edit" | "delete" | "export" | "import" | "custom";
  permission: string | null;
  visible: boolean;
  disabled: boolean;
  sort: number;
}

export interface TableRowAction {
  key: string;
  name: string;
  icon: string;
  type: "primary" | "success" | "warning" | "danger" | "info" | "default";
  action: "view" | "edit" | "delete" | "copy" | "publish" | "unpublish" | "custom";
  permission: string | null;
  visible: boolean;
  disabled: boolean;
  confirm: string | null;
  sort: number;
}

export interface TablePagination {
  enabled: boolean;
  pageSize: number;
  pageSizes: number[];
  layout: string;
}

export interface TableSchema {
  columns: TableColumn[];
  search: TableSearchConfig;
  toolbar: TableToolbarButton[];
  rowActions: TableRowAction[];
  pagination: TablePagination;
  selection: {
    enabled: boolean;
    type: "checkbox" | "radio";
  };
  expand: {
    enabled: boolean;
    field: string | null;
  };
}

export interface TableConfigItem {
  id: number;
  name: string;
  code: string;
  modelId: number;
  modelName: string;
  description: string | null;
  isEnabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export const tableConfigList: TableConfigItem[] = [
  {
    id: 1,
    name: "文章列表配置",
    code: "article_list",
    modelId: 1,
    modelName: "文章",
    description: "文章内容列表表格配置",
    isEnabled: true,
    createdAt: "2024-01-01 00:00:00",
    updatedAt: "2024-01-01 00:00:00"
  },
  {
    id: 2,
    name: "产品列表配置",
    code: "product_list",
    modelId: 2,
    modelName: "产品",
    description: "产品内容列表表格配置",
    isEnabled: true,
    createdAt: "2024-01-02 00:00:00",
    updatedAt: "2024-01-02 00:00:00"
  },
  {
    id: 3,
    name: "模型列表配置",
    code: "model_list",
    modelId: 0,
    modelName: "系统",
    description: "内容模型列表表格配置",
    isEnabled: true,
    createdAt: "2024-01-03 00:00:00",
    updatedAt: "2024-01-03 00:00:00"
  }
];

export const tableSchema: TableSchema = {
  columns: [
    {
      field: "selection",
      title: "选择",
      width: 50,
      minWidth: null,
      align: "center",
      fixed: "left",
      sortable: false,
      visible: true,
      sort: 0,
      formatter: null,
      render: null
    },
    {
      field: "id",
      title: "ID",
      width: 80,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 1,
      formatter: null,
      render: null
    },
    {
      field: "thumbnail",
      title: "缩略图",
      width: 100,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: false,
      visible: true,
      sort: 2,
      formatter: null,
      render: "image"
    },
    {
      field: "title",
      title: "标题",
      width: 300,
      minWidth: 200,
      align: "left",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 3,
      formatter: null,
      render: "link"
    },
    {
      field: "category_name",
      title: "所属分类",
      width: 120,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: false,
      visible: true,
      sort: 4,
      formatter: null,
      render: "tag"
    },
    {
      field: "model_name",
      title: "内容模型",
      width: 100,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: false,
      visible: true,
      sort: 5,
      formatter: null,
      render: "tag"
    },
    {
      field: "status",
      title: "状态",
      width: 100,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 6,
      formatter: null,
      render: "status"
    },
    {
      field: "is_top",
      title: "置顶",
      width: 80,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 7,
      formatter: null,
      render: "switch"
    },
    {
      field: "is_recommend",
      title: "推荐",
      width: 80,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 8,
      formatter: null,
      render: "switch"
    },
    {
      field: "view_count",
      title: "阅读量",
      width: 100,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 9,
      formatter: "number",
      render: null
    },
    {
      field: "author",
      title: "作者",
      width: 100,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 10,
      formatter: null,
      render: null
    },
    {
      field: "publish_time",
      title: "发布时间",
      width: 160,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: true,
      sort: 11,
      formatter: "datetime",
      render: null
    },
    {
      field: "created_at",
      title: "创建时间",
      width: 160,
      minWidth: null,
      align: "center",
      fixed: null,
      sortable: true,
      visible: false,
      sort: 12,
      formatter: "datetime",
      render: null
    },
    {
      field: "actions",
      title: "操作",
      width: 200,
      minWidth: null,
      align: "center",
      fixed: "right",
      sortable: false,
      visible: true,
      sort: 99,
      formatter: null,
      render: "actions"
    }
  ],
  search: {
    enabled: true,
    fields: [
      {
        field: "title",
        title: "标题",
        searchType: "like",
        fieldType: "text",
        options: null,
        placeholder: "请输入标题关键词",
        defaultValue: null,
        weight: 1
      },
      {
        field: "category_id",
        title: "所属分类",
        searchType: "eq",
        fieldType: "tree-select",
        options: null,
        placeholder: "请选择分类",
        defaultValue: null,
        weight: 2
      },
      {
        field: "model_id",
        title: "内容模型",
        searchType: "eq",
        fieldType: "select",
        options: [
          { label: "文章", value: 1 },
          { label: "产品", value: 2 },
          { label: "图片", value: 3 },
          { label: "视频", value: 4 },
          { label: "下载", value: 5 }
        ],
        placeholder: "请选择模型",
        defaultValue: null,
        weight: 3
      },
      {
        field: "status",
        title: "状态",
        searchType: "eq",
        fieldType: "select",
        options: [
          { label: "草稿", value: "draft" },
          { label: "已发布", value: "published" },
          { label: "未发布", value: "unpublished" },
          { label: "已归档", value: "archived" }
        ],
        placeholder: "请选择状态",
        defaultValue: null,
        weight: 4
      },
      {
        field: "publish_time",
        title: "发布时间",
        searchType: "daterange",
        fieldType: "daterange",
        options: null,
        placeholder: "请选择时间范围",
        defaultValue: null,
        weight: 5
      },
      {
        field: "is_top",
        title: "是否置顶",
        searchType: "eq",
        fieldType: "select",
        options: [
          { label: "是", value: 1 },
          { label: "否", value: 0 }
        ],
        placeholder: "请选择",
        defaultValue: null,
        weight: 6
      },
      {
        field: "is_recommend",
        title: "是否推荐",
        searchType: "eq",
        fieldType: "select",
        options: [
          { label: "是", value: 1 },
          { label: "否", value: 0 }
        ],
        placeholder: "请选择",
        defaultValue: null,
        weight: 7
      }
    ],
    placeholder: "请输入关键词搜索",
    showCollapse: true,
    defaultCollapsed: false
  },
  toolbar: [
    {
      key: "create",
      name: "新增",
      icon: "Plus",
      type: "primary",
      action: "create",
      permission: "cms:content:create",
      visible: true,
      disabled: false,
      sort: 1
    },
    {
      key: "delete",
      name: "批量删除",
      icon: "Delete",
      type: "danger",
      action: "delete",
      permission: "cms:content:delete",
      visible: true,
      disabled: false,
      sort: 2
    },
    {
      key: "export",
      name: "导出",
      icon: "Download",
      type: "success",
      action: "export",
      permission: "cms:content:export",
      visible: true,
      disabled: false,
      sort: 3
    },
    {
      key: "import",
      name: "导入",
      icon: "Upload",
      type: "warning",
      action: "import",
      permission: "cms:content:import",
      visible: true,
      disabled: false,
      sort: 4
    }
  ],
  rowActions: [
    {
      key: "view",
      name: "查看",
      icon: "View",
      type: "primary",
      action: "view",
      permission: "cms:content:view",
      visible: true,
      disabled: false,
      confirm: null,
      sort: 1
    },
    {
      key: "edit",
      name: "编辑",
      icon: "Edit",
      type: "primary",
      action: "edit",
      permission: "cms:content:edit",
      visible: true,
      disabled: false,
      confirm: null,
      sort: 2
    },
    {
      key: "publish",
      name: "发布",
      icon: "Upload",
      type: "success",
      action: "publish",
      permission: "cms:content:publish",
      visible: true,
      disabled: false,
      confirm: "确定要发布该内容吗？",
      sort: 3
    },
    {
      key: "unpublish",
      name: "取消发布",
      icon: "Download",
      type: "warning",
      action: "unpublish",
      permission: "cms:content:publish",
      visible: true,
      disabled: false,
      confirm: "确定要取消发布该内容吗？",
      sort: 4
    },
    {
      key: "copy",
      name: "复制",
      icon: "CopyDocument",
      type: "info",
      action: "copy",
      permission: "cms:content:create",
      visible: true,
      disabled: false,
      confirm: "确定要复制该内容吗？",
      sort: 5
    },
    {
      key: "delete",
      name: "删除",
      icon: "Delete",
      type: "danger",
      action: "delete",
      permission: "cms:content:delete",
      visible: true,
      disabled: false,
      confirm: "确定要删除该内容吗？删除后不可恢复！",
      sort: 6
    }
  ],
  pagination: {
    enabled: true,
    pageSize: 20,
    pageSizes: [10, 20, 50, 100],
    layout: "total, sizes, prev, pager, next, jumper"
  },
  selection: {
    enabled: true,
    type: "checkbox"
  },
  expand: {
    enabled: false,
    field: null
  }
};
