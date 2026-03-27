import type { MockMethod } from "vite-plugin-mock";
import { resultSuccess, resultError } from "../_utils";
import { modelList, modelDetail, fieldList } from "./model.mock";
import { contentList, contentDetail } from "./content.mock";
import { categoryTree, categoryList } from "./category.mock";
import { tagList, tagCloud } from "./tag.mock";
import { formConfigList, formSchema } from "./form.mock";
import { tableConfigList, tableSchema } from "./table.mock";
import { generatedFiles, fileTree, codeGenConfig } from "./codeGen.mock";

export * from "./model.mock";
export * from "./content.mock";
export * from "./category.mock";
export * from "./tag.mock";
export * from "./form.mock";
export * from "./table.mock";
export * from "./codeGen.mock";

const cmsModule: MockMethod[] = [
  {
    url: "/mock/cms/model/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { page = 1, pageSize = 10, name, isEnabled } = query;
      let list = [...modelList];
      
      if (name) {
        list = list.filter(item => item.name.includes(name) || item.code.includes(name));
      }
      if (isEnabled !== undefined) {
        list = list.filter(item => item.isEnabled === (isEnabled === 'true' || isEnabled === true));
      }
      
      const total = list.length;
      const start = (page - 1) * pageSize;
      const end = start + pageSize;
      const data = list.slice(start, end);
      
      return resultSuccess({
        list: data,
        total,
        pageNum: Number(page),
        pageSize: Number(pageSize)
      });
    }
  },
  {
    url: "/mock/cms/model/detail",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { id } = query;
      if (!id) {
        return resultError(null, "模型ID不能为空", 400);
      }
      
      const model = modelList.find(item => item.id === Number(id));
      if (!model) {
        return resultError(null, "模型不存在", 404);
      }
      
      const detail = {
        ...modelDetail,
        id: model.id,
        name: model.name,
        code: model.code,
        tableName: model.tableName,
        description: model.description,
        icon: model.icon,
        isEnabled: model.isEnabled,
        isSystem: model.isSystem,
        sort: model.sort,
        fields: fieldList.filter(f => f.modelId === Number(id))
      };
      
      return resultSuccess(detail);
    }
  },
  {
    url: "/mock/cms/model/add",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess({ id: Date.now() });
    }
  },
  {
    url: "/mock/cms/model/edit",
    method: "put",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/model/delete",
    method: "delete",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/model/field/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { modelId } = query;
      let list = [...fieldList];
      
      if (modelId) {
        list = list.filter(item => item.modelId === Number(modelId));
      }
      
      return resultSuccess(list);
    }
  },
  {
    url: "/mock/cms/content/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { page = 1, pageSize = 10, title, categoryId, modelId, status } = query;
      let list = [...contentList];
      
      if (title) {
        list = list.filter(item => item.title.includes(title));
      }
      if (categoryId) {
        list = list.filter(item => item.categoryId === Number(categoryId));
      }
      if (modelId) {
        list = list.filter(item => item.modelId === Number(modelId));
      }
      if (status) {
        list = list.filter(item => item.status === status);
      }
      
      const total = list.length;
      const start = (page - 1) * pageSize;
      const end = start + pageSize;
      const data = list.slice(start, end);
      
      return resultSuccess({
        list: data,
        total,
        pageNum: Number(page),
        pageSize: Number(pageSize)
      });
    }
  },
  {
    url: "/mock/cms/content/detail",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { id } = query;
      if (!id) {
        return resultError(null, "内容ID不能为空", 400);
      }
      
      const content = contentList.find(item => item.id === Number(id));
      if (!content) {
        return resultError(null, "内容不存在", 404);
      }
      
      return resultSuccess(contentDetail);
    }
  },
  {
    url: "/mock/cms/content/add",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess({ id: Date.now() });
    }
  },
  {
    url: "/mock/cms/content/edit",
    method: "put",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/content/delete",
    method: "delete",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/content/publish",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/content/unpublish",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/category/tree",
    method: "get",
    timeout: 300,
    response: () => {
      return resultSuccess(categoryTree);
    }
  },
  {
    url: "/mock/cms/category/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { page = 1, pageSize = 10, name, status } = query;
      let list = [...categoryList];
      
      if (name) {
        list = list.filter(item => item.name.includes(name) || item.code.includes(name));
      }
      if (status !== undefined) {
        list = list.filter(item => item.status === (status === 'true' || status === true));
      }
      
      const total = list.length;
      const start = (page - 1) * pageSize;
      const end = start + pageSize;
      const data = list.slice(start, end);
      
      return resultSuccess({
        list: data,
        total,
        pageNum: Number(page),
        pageSize: Number(pageSize)
      });
    }
  },
  {
    url: "/mock/cms/category/add",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess({ id: Date.now() });
    }
  },
  {
    url: "/mock/cms/category/edit",
    method: "put",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/category/delete",
    method: "delete",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/tag/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { page = 1, pageSize = 10, name, status } = query;
      let list = [...tagList];
      
      if (name) {
        list = list.filter(item => item.name.includes(name) || item.code.includes(name));
      }
      if (status !== undefined) {
        list = list.filter(item => item.status === (status === 'true' || status === true));
      }
      
      const total = list.length;
      const start = (page - 1) * pageSize;
      const end = start + pageSize;
      const data = list.slice(start, end);
      
      return resultSuccess({
        list: data,
        total,
        pageNum: Number(page),
        pageSize: Number(pageSize)
      });
    }
  },
  {
    url: "/mock/cms/tag/cloud",
    method: "get",
    timeout: 300,
    response: () => {
      return resultSuccess(tagCloud);
    }
  },
  {
    url: "/mock/cms/tag/add",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess({ id: Date.now() });
    }
  },
  {
    url: "/mock/cms/tag/edit",
    method: "put",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/tag/delete",
    method: "delete",
    timeout: 300,
    response: () => {
      return resultSuccess(null);
    }
  },
  {
    url: "/mock/cms/form/config/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { page = 1, pageSize = 10, modelId, formType } = query;
      let list = [...formConfigList];
      
      if (modelId) {
        list = list.filter(item => item.modelId === Number(modelId));
      }
      if (formType) {
        list = list.filter(item => item.formType === formType);
      }
      
      const total = list.length;
      const start = (page - 1) * pageSize;
      const end = start + pageSize;
      const data = list.slice(start, end);
      
      return resultSuccess({
        list: data,
        total,
        pageNum: Number(page),
        pageSize: Number(pageSize)
      });
    }
  },
  {
    url: "/mock/cms/form/schema",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { modelId, formType = 'create' } = query;
      return resultSuccess({
        ...formSchema,
        formType
      });
    }
  },
  {
    url: "/mock/cms/form/config/save",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess({ id: Date.now() });
    }
  },
  {
    url: "/mock/cms/table/config/list",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { page = 1, pageSize = 10, modelId } = query;
      let list = [...tableConfigList];
      
      if (modelId) {
        list = list.filter(item => item.modelId === Number(modelId));
      }
      
      const total = list.length;
      const start = (page - 1) * pageSize;
      const end = start + pageSize;
      const data = list.slice(start, end);
      
      return resultSuccess({
        list: data,
        total,
        pageNum: Number(page),
        pageSize: Number(pageSize)
      });
    }
  },
  {
    url: "/mock/cms/table/schema",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { modelId } = query;
      return resultSuccess(tableSchema);
    }
  },
  {
    url: "/mock/cms/table/config/save",
    method: "post",
    timeout: 300,
    response: () => {
      return resultSuccess({ id: Date.now() });
    }
  },
  {
    url: "/mock/cms/codegen/preview",
    method: "post",
    timeout: 500,
    response: ({ body }) => {
      return resultSuccess({
        files: generatedFiles,
        fileTree
      });
    }
  },
  {
    url: "/mock/cms/codegen/generate",
    method: "post",
    timeout: 1000,
    response: ({ body }) => {
      return resultSuccess({
        success: true,
        message: "代码生成成功"
      });
    }
  },
  {
    url: "/mock/cms/codegen/download",
    method: "post",
    timeout: 500,
    response: ({ body }) => {
      return resultSuccess({
        downloadUrl: "/api/cms/codegen/download/file.zip"
      });
    }
  },
  {
    url: "/mock/cms/codegen/config",
    method: "get",
    timeout: 300,
    response: ({ query }) => {
      const { modelId } = query;
      return resultSuccess(codeGenConfig);
    }
  }
];

export default cmsModule;
