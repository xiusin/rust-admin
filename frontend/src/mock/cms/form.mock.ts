export interface FormFieldConfig {
  field: string;
  title: string;
  fieldType: 'text' | 'textarea' | 'number' | 'select' | 'radio' | 'checkbox' | 'date' | 'datetime' | 'image' | 'file' | 'editor' | 'switch' | 'cascader' | 'tree-select';
  defaultValue: string | number | boolean | null;
  placeholder: string | null;
  required: boolean;
  disabled: boolean;
  readonly: boolean;
  visible: boolean;
  options: { label: string; value: string | number }[] | null;
  validation: {
    rules: string[];
    message: string;
  } | null;
  span: number;
  helpText: string | null;
  extra: Record<string, unknown> | null;
}

export interface FormGroup {
  id: string;
  name: string;
  code: string;
  fields: string[];
  visible: boolean;
  collapsed: boolean;
  sort: number;
}

export interface FormLayout {
  layoutType: 'horizontal' | 'vertical' | 'inline';
  columns: number;
  labelWidth: string;
  labelPosition: 'left' | 'right' | 'top';
  size: 'large' | 'default' | 'small';
}

export interface FormSchema {
  formType: 'create' | 'edit' | 'view';
  layout: FormLayout;
  groups: FormGroup[];
  fields: FormFieldConfig[];
}

export interface FormConfigItem {
  id: number;
  name: string;
  code: string;
  modelId: number;
  modelName: string;
  formType: 'create' | 'edit' | 'view';
  description: string | null;
  isEnabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export const formConfigList: FormConfigItem[] = [
  {
    id: 1,
    name: '文章创建表单',
    code: 'article_create',
    modelId: 1,
    modelName: '文章',
    formType: 'create',
    description: '文章内容创建表单配置',
    isEnabled: true,
    createdAt: '2024-01-01 00:00:00',
    updatedAt: '2024-01-01 00:00:00'
  },
  {
    id: 2,
    name: '文章编辑表单',
    code: 'article_edit',
    modelId: 1,
    modelName: '文章',
    formType: 'edit',
    description: '文章内容编辑表单配置',
    isEnabled: true,
    createdAt: '2024-01-01 00:00:00',
    updatedAt: '2024-01-01 00:00:00'
  },
  {
    id: 3,
    name: '产品创建表单',
    code: 'product_create',
    modelId: 2,
    modelName: '产品',
    formType: 'create',
    description: '产品内容创建表单配置',
    isEnabled: true,
    createdAt: '2024-01-02 00:00:00',
    updatedAt: '2024-01-02 00:00:00'
  },
  {
    id: 4,
    name: '产品编辑表单',
    code: 'product_edit',
    modelId: 2,
    modelName: '产品',
    formType: 'edit',
    description: '产品内容编辑表单配置',
    isEnabled: true,
    createdAt: '2024-01-02 00:00:00',
    updatedAt: '2024-01-02 00:00:00'
  }
];

export const formSchema: FormSchema = {
  formType: 'create',
  layout: {
    layoutType: 'horizontal',
    columns: 2,
    labelWidth: '100px',
    labelPosition: 'right',
    size: 'default'
  },
  groups: [
    {
      id: 'basic',
      name: '基本信息',
      code: 'basic',
      fields: ['title', 'subtitle', 'category_id', 'thumbnail'],
      visible: true,
      collapsed: false,
      sort: 1
    },
    {
      id: 'content',
      name: '内容详情',
      code: 'content',
      fields: ['content', 'summary'],
      visible: true,
      collapsed: false,
      sort: 2
    },
    {
      id: 'attributes',
      name: '属性设置',
      code: 'attributes',
      fields: ['author', 'source', 'tags', 'status', 'publish_time'],
      visible: true,
      collapsed: false,
      sort: 3
    },
    {
      id: 'seo',
      name: 'SEO设置',
      code: 'seo',
      fields: ['seo_title', 'seo_keywords', 'seo_description'],
      visible: true,
      collapsed: true,
      sort: 4
    },
    {
      id: 'advanced',
      name: '高级设置',
      code: 'advanced',
      fields: ['is_top', 'is_recommend', 'view_count'],
      visible: true,
      collapsed: true,
      sort: 5
    }
  ],
  fields: [
    {
      field: 'title',
      title: '标题',
      fieldType: 'text',
      defaultValue: null,
      placeholder: '请输入标题',
      required: true,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['required', 'max:200'],
        message: '标题不能为空且长度不超过200字符'
      },
      span: 2,
      helpText: '标题长度不超过200字符',
      extra: null
    },
    {
      field: 'subtitle',
      title: '副标题',
      fieldType: 'text',
      defaultValue: null,
      placeholder: '请输入副标题',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:200'],
        message: '副标题长度不超过200字符'
      },
      span: 2,
      helpText: null,
      extra: null
    },
    {
      field: 'category_id',
      title: '所属分类',
      fieldType: 'tree-select',
      defaultValue: null,
      placeholder: '请选择分类',
      required: true,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['required'],
        message: '请选择所属分类'
      },
      span: 1,
      helpText: null,
      extra: {
        treeData: 'category',
        checkStrictly: true
      }
    },
    {
      field: 'thumbnail',
      title: '缩略图',
      fieldType: 'image',
      defaultValue: null,
      placeholder: null,
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: null,
      span: 1,
      helpText: '支持jpg、png、gif格式，最大2MB',
      extra: {
        maxSize: 2048,
        accept: '.jpg,.jpeg,.png,.gif',
        limit: 1
      }
    },
    {
      field: 'content',
      title: '内容',
      fieldType: 'editor',
      defaultValue: null,
      placeholder: '请输入内容',
      required: true,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['required'],
        message: '内容不能为空'
      },
      span: 2,
      helpText: null,
      extra: {
        height: 400,
        placeholder: '请输入内容'
      }
    },
    {
      field: 'summary',
      title: '摘要',
      fieldType: 'textarea',
      defaultValue: null,
      placeholder: '请输入摘要',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:500'],
        message: '摘要长度不超过500字符'
      },
      span: 2,
      helpText: '摘要长度不超过500字符',
      extra: {
        rows: 4,
        maxlength: 500,
        showWordLimit: true
      }
    },
    {
      field: 'author',
      title: '作者',
      fieldType: 'text',
      defaultValue: '管理员',
      placeholder: '请输入作者',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:50'],
        message: '作者长度不超过50字符'
      },
      span: 1,
      helpText: null,
      extra: null
    },
    {
      field: 'source',
      title: '来源',
      fieldType: 'text',
      defaultValue: null,
      placeholder: '请输入来源',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:100'],
        message: '来源长度不超过100字符'
      },
      span: 1,
      helpText: null,
      extra: null
    },
    {
      field: 'tags',
      title: '标签',
      fieldType: 'select',
      defaultValue: null,
      placeholder: '请选择标签',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: [
        { label: '热门', value: 'hot' },
        { label: '推荐', value: 'recommend' },
        { label: '技术', value: 'tech' },
        { label: '教程', value: 'tutorial' },
        { label: '行业', value: 'industry' }
      ],
      validation: null,
      span: 1,
      helpText: null,
      extra: {
        multiple: true,
        filterable: true
      }
    },
    {
      field: 'status',
      title: '状态',
      fieldType: 'radio',
      defaultValue: 'draft',
      placeholder: null,
      required: true,
      disabled: false,
      readonly: false,
      visible: true,
      options: [
        { label: '草稿', value: 'draft' },
        { label: '已发布', value: 'published' },
        { label: '未发布', value: 'unpublished' },
        { label: '已归档', value: 'archived' }
      ],
      validation: {
        rules: ['required'],
        message: '请选择状态'
      },
      span: 1,
      helpText: null,
      extra: null
    },
    {
      field: 'publish_time',
      title: '发布时间',
      fieldType: 'datetime',
      defaultValue: null,
      placeholder: '请选择发布时间',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: null,
      span: 1,
      helpText: null,
      extra: {
        format: 'YYYY-MM-DD HH:mm:ss',
        valueFormat: 'YYYY-MM-DD HH:mm:ss'
      }
    },
    {
      field: 'seo_title',
      title: 'SEO标题',
      fieldType: 'text',
      defaultValue: null,
      placeholder: '请输入SEO标题',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:100'],
        message: 'SEO标题长度不超过100字符'
      },
      span: 2,
      helpText: null,
      extra: null
    },
    {
      field: 'seo_keywords',
      title: 'SEO关键词',
      fieldType: 'text',
      defaultValue: null,
      placeholder: '请输入SEO关键词，多个用逗号分隔',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:200'],
        message: 'SEO关键词长度不超过200字符'
      },
      span: 2,
      helpText: null,
      extra: null
    },
    {
      field: 'seo_description',
      title: 'SEO描述',
      fieldType: 'textarea',
      defaultValue: null,
      placeholder: '请输入SEO描述',
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['max:300'],
        message: 'SEO描述长度不超过300字符'
      },
      span: 2,
      helpText: null,
      extra: {
        rows: 3,
        maxlength: 300,
        showWordLimit: true
      }
    },
    {
      field: 'is_top',
      title: '是否置顶',
      fieldType: 'switch',
      defaultValue: false,
      placeholder: null,
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: null,
      span: 1,
      helpText: null,
      extra: null
    },
    {
      field: 'is_recommend',
      title: '是否推荐',
      fieldType: 'switch',
      defaultValue: false,
      placeholder: null,
      required: false,
      disabled: false,
      readonly: false,
      visible: true,
      options: null,
      validation: null,
      span: 1,
      helpText: null,
      extra: null
    },
    {
      field: 'view_count',
      title: '阅读量',
      fieldType: 'number',
      defaultValue: 0,
      placeholder: null,
      required: false,
      disabled: true,
      readonly: false,
      visible: true,
      options: null,
      validation: {
        rules: ['min:0'],
        message: '阅读量不能小于0'
      },
      span: 1,
      helpText: '自动统计，不可编辑',
      extra: {
        min: 0,
        precision: 0
      }
    }
  ]
};
