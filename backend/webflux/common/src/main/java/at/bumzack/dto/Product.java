package at.bumzack.dto;

import at.bumzack.search.SearchResult;
import at.bumzack.solr.SolrResponse;
import at.bumzack.solr.SolrResponseResponse;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Product extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Product>> TYPE_REF_PRODUCT = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Product>> TYPE_REF_SEARCH_RESULT_PRODUCT= new ParameterizedTypeReference<>() {
    };

    private static final long serialVersionUID = 3L;

    private String article;
    private String articleUnit;
    private String articleName;
    private String articleDescription;
    private boolean visible;
    private boolean orderable;
    private String sourcing;
    private String division;
    private String material;
    private String codeWhg;
    private String supplierName;
    private String defaultSupplier;
    private List<String> otns;
    private List<String> eans;
    private List<String> predecessorCodes;
    private List<String> predecessorEans;
    private List<String> predecessorOtns;
    private String module;
    private String moduleGroup;
    private String ownBrand;
    private List<String> superCategories;
    private List<String> allSuperCategories;

    private List<String> imagesContainerQualifiers;
    private String mainImageContainerQualifier;

    public Product() {
    }

    public String getArticle() {
        return article;
    }

    public void setArticle(final String article) {
        this.article = article;
    }

    public String getArticleUnit() {
        return articleUnit;
    }

    public void setArticleUnit(final String articleUnit) {
        this.articleUnit = articleUnit;
    }

    public String getArticleName() {
        return articleName;
    }

    public void setArticleName(final String articleName) {
        this.articleName = articleName;
    }

    public String getArticleDescription() {
        return articleDescription;
    }

    public void setArticleDescription(final String articleDescription) {
        this.articleDescription = articleDescription;
    }

    public boolean isVisible() {
        return visible;
    }

    public void setVisible(final boolean visible) {
        this.visible = visible;
    }

    public boolean isOrderable() {
        return orderable;
    }

    public void setOrderable(final boolean orderable) {
        this.orderable = orderable;
    }

    public String getSourcing() {
        return sourcing;
    }

    public void setSourcing(final String sourcing) {
        this.sourcing = sourcing;
    }

    public String getDivision() {
        return division;
    }

    public void setDivision(final String division) {
        this.division = division;
    }

    public String getMaterial() {
        return material;
    }

    public void setMaterial(final String material) {
        this.material = material;
    }

    public String getCodeWhg() {
        return codeWhg;
    }

    public void setCodeWhg(final String codeWhg) {
        this.codeWhg = codeWhg;
    }

    public String getSupplierName() {
        return supplierName;
    }

    public void setSupplierName(final String supplierName) {
        this.supplierName = supplierName;
    }

    public String getDefaultSupplier() {
        return defaultSupplier;
    }

    public void setDefaultSupplier(final String defaultSupplier) {
        this.defaultSupplier = defaultSupplier;
    }

    public List<String> getOtns() {
        return otns;
    }

    public void setOtns(final List<String> otns) {
        this.otns = otns;
    }

    public List<String> getEans() {
        return eans;
    }

    public void setEans(final List<String> eans) {
        this.eans = eans;
    }

    public List<String> getPredecessorCodes() {
        return predecessorCodes;
    }

    public void setPredecessorCodes(final List<String> predecessorCodes) {
        this.predecessorCodes = predecessorCodes;
    }

    public List<String> getPredecessorEans() {
        return predecessorEans;
    }

    public void setPredecessorEans(final List<String> predecessorEans) {
        this.predecessorEans = predecessorEans;
    }

    public List<String> getPredecessorOtns() {
        return predecessorOtns;
    }

    public void setPredecessorOtns(final List<String> predecessorOtns) {
        this.predecessorOtns = predecessorOtns;
    }

    public String getModule() {
        return module;
    }

    public void setModule(final String module) {
        this.module = module;
    }

    public String getModuleGroup() {
        return moduleGroup;
    }

    public void setModuleGroup(final String moduleGroup) {
        this.moduleGroup = moduleGroup;
    }

    public String getOwnBrand() {
        return ownBrand;
    }

    public void setOwnBrand(final String ownBrand) {
        this.ownBrand = ownBrand;
    }

    public List<String> getSuperCategories() {
        return superCategories;
    }

    public void setSuperCategories(final List<String> superCategories) {
        this.superCategories = superCategories;
    }

    public List<String> getAllSuperCategories() {
        return allSuperCategories;
    }

    public void setAllSuperCategories(final List<String> allSuperCategories) {
        this.allSuperCategories = allSuperCategories;
    }

    public List<String> getImagesContainerQualifiers() {
        return imagesContainerQualifiers;
    }

    public void setImagesContainerQualifiers(final List<String> imagesContainerQualifiers) {
        this.imagesContainerQualifiers = imagesContainerQualifiers;
    }

    public String getMainImageContainerQualifier() {
        return mainImageContainerQualifier;
    }

    public void setMainImageContainerQualifier(final String mainImageContainerQualifier) {
        this.mainImageContainerQualifier = mainImageContainerQualifier;
    }

}
